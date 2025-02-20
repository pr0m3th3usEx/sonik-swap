use std::time::Duration;

use oauth2::http::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use snk_core::{
    contracts::repositories::provider_account_repository::{
        ProviderAccountRepository, ProviderAccountRepositoryError, ProviderAccountRepositoryResult,
    },
    entities::provider_account::ProviderAccount,
};

use crate::deezer::{DeezerResponse, API_URL};

pub struct DeezerProviderAccountRepository {
    http_client: Client,
}

impl DeezerProviderAccountRepository {
    pub fn new(access_token: String) -> Result<Self, &'static str> {
        let mut default_headers = HeaderMap::new();

        default_headers.insert("Accept", HeaderValue::from_static("application/json"));
        default_headers.insert(
            "Authorization",
            format!("Bearer: {}", access_token).parse().map_err(|err| {
                eprintln!("{:?}", err);
                "DeezerProviderAccountRepository::new: Could not parse header value"
            })?,
        );

        Ok(Self {
            http_client: Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .default_headers(default_headers)
                .build()
                .map_err(|err| {
                    eprintln!("{:?}", err);
                    "DeezerProviderAccountRepository::new: Could not init HTTP client"
                })?,
        })
    }
}

impl ProviderAccountRepository for DeezerProviderAccountRepository {
    async fn get_logged_user(&self) -> ProviderAccountRepositoryResult<ProviderAccount> {
        let url = format!("{}/", API_URL);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|err| ProviderAccountRepositoryError::ServiceError(err.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response
                    .json::<DeezerResponse>()
                    .await
                    .map_err(|err| ProviderAccountRepositoryError::ServiceError(err.to_string()))?;

                match response_body {
                    DeezerResponse::Error(deezer_error_payload) => {
                        Err(ProviderAccountRepositoryError::ServiceError(
                            deezer_error_payload.error.message,
                        ))
                    }
                    DeezerResponse::User(user) => Ok(user.into()),
                    _ => Err(ProviderAccountRepositoryError::ServiceError(
                        "bad response format".to_string(),
                    )),
                }
            }
            other => Err(ProviderAccountRepositoryError::ServiceError(format!(
                "Failed request: {}",
                other
            ))),
        }
    }
}
