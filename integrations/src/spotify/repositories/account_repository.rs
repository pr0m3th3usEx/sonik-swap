use std::time::Duration;

use oauth2::http::{HeaderMap, HeaderValue};
use reqwest::Client;
use snk_core::{
    contracts::repositories::provider_account_repository::{
        ProviderAccountRepository, ProviderAccountRepositoryError, ProviderAccountRepositoryResult,
    },
    entities::provider_account::ProviderAccount,
};

use crate::spotify::{user::SpotifyUser, API_URL};

pub struct SpotifyProviderAccountRepository {
    http_client: Client,
}

impl SpotifyProviderAccountRepository {
    pub fn new(access_token: String) -> Result<Self, &'static str> {
        let mut default_headers = HeaderMap::new();

        default_headers.insert("Accept", HeaderValue::from_static("application/json"));
        default_headers.insert(
            "Authorization",
            format!("Bearer: {}", access_token).parse().map_err(|err| {
                eprintln!("{:?}", err);
                "SpotifyProviderAccountRepository::new: Could not parse header value"
            })?,
        );

        Ok(Self {
            http_client: Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .default_headers(default_headers)
                .build()
                .map_err(|err| {
                    eprintln!("{:?}", err);
                    "SpotifyProviderAccountRepository::new: Could not init HTTP client"
                })?,
        })
    }
}

impl ProviderAccountRepository for SpotifyProviderAccountRepository {
    async fn get_logged_user(&self) -> ProviderAccountRepositoryResult<ProviderAccount> {
        let url = format!("{}/me", API_URL);

        let response =  self.http_client
          .get(url)
          .send()
          .await
          .map_err(|err| {
            ProviderAccountRepositoryError::ServiceError(
                format!("SpotifyProviderAccountRepository::get_logged_user: error while sending request: {}", err.to_string()
            ))
        })?;

        match response.error_for_status() {
            Ok(response) => {
                let user = response.json::<SpotifyUser>().await.map_err(|err| {
                    ProviderAccountRepositoryError::ServiceError(format!(
                        "ProviderAccountRepositoryError - Failed to parse response - {:?}",
                        err
                    ))
                })?;

                Ok(user.into())
            }
            Err(err) => Err(ProviderAccountRepositoryError::ServiceError(format!(
                "ProviderAccountRepositoryError - Error during request - {:?}",
                err
            ))),
        }
    }
}
