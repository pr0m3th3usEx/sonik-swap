use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest, AuthorizationCode, Client, ClientId, ClientSecret, RedirectUrl, RequestTokenError,
    StandardErrorResponse, TokenResponse,
};
use thiserror::Error;

use crate::{
    contracts::{
        providers::token_provider::TokenProvider,
        repositories::{
            music_account_provider_repository::{
                MusicAccountProviderRepository, MusicAccountProviderRepositoryError,
            },
            user_repository::UserRepository,
        },
    },
    value_objects::provider::provider_id::ProviderId,
};

pub struct LoginOAuth2CallbackQuery {
    provider_id: ProviderId,
    redirect_url: RedirectUrl,
    code: AuthorizationCode,
}

#[derive(Debug, Error)]
pub enum LoginOAuth2CallbackQueryError {
    #[error("Provider not found")]
    ProviderNotFound,
    #[error("Authentication with {0} unavailable")]
    OAuth2AuthenticationUnvailable(String),
    #[error("InternalError: {0}")]
    InternalError(String),
}

impl From<MusicAccountProviderRepositoryError> for LoginOAuth2CallbackQueryError {
    fn from(error: MusicAccountProviderRepositoryError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl LoginOAuth2CallbackQuery {
    pub fn new(
        provider_id: ProviderId,
        redirect_url: RedirectUrl,
        code: AuthorizationCode,
    ) -> Self {
        Self {
            provider_id,
            redirect_url,
            code,
        }
    }

    pub async fn execute(
        self,
        user_repo: &impl UserRepository,
        auth_repo: &impl MusicAccountProviderRepository,
        access_token_provider: &impl TokenProvider,
        refresh_token_provider: &impl TokenProvider,
    ) -> Result<(), LoginOAuth2CallbackQueryError> {
        let (client_id, client_secret) = match self.provider_id.as_str() {
            "spotify" => (
                ClientId::new(std::env::var("SPOTIFY_OAUTH2_CLIENT_ID").unwrap()),
                ClientSecret::new(std::env::var("SPOTIFY_OAUTH2_CLIENT_SECRET").unwrap()),
            ),
            "deezer" => (
                ClientId::new(std::env::var("DEEZER_OAUTH2_CLIENT_ID").unwrap()),
                ClientSecret::new(std::env::var("DEEZER_OAUTH2_CLIENT_SECRET").unwrap()),
            ),
            _ => return Err(LoginOAuth2CallbackQueryError::ProviderNotFound),
        };

        let Some(auth_provider) = auth_repo.get(&self.provider_id).await? else {
            return Err(LoginOAuth2CallbackQueryError::ProviderNotFound);
        };

        if !auth_provider.authentication_allowed {
            return Err(
                LoginOAuth2CallbackQueryError::OAuth2AuthenticationUnvailable(
                    auth_provider.name.value(),
                ),
            );
        }

        // Create oauth2 client
        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_token_uri(auth_provider.token_url)
            .set_redirect_uri(self.redirect_url);

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // Run authorization request from token url
        let token_response = client
            .exchange_code(self.code)
            .request_async(&http_client)
            .await
            .map_err(|err| {
                tracing::error!({ ?err }, "Error while granting tokens");
                LoginOAuth2CallbackQueryError::InternalError(err.to_string())
            })?;

        tracing::debug!("Extra fields: {:?}", token_response.extra_fields());
        tracing::debug!("Access token: {:?}", token_response.access_token());
        tracing::debug!("Refresh token: {:?}", token_response.refresh_token());

        // Get profile information
        // Get user account with oauth2 allowed methods
        // Check if provider method is allowed

        Ok(())
    }
}
