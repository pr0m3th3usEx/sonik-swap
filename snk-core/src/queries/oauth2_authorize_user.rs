use oauth2::{basic::BasicClient, ClientId, ClientSecret, CsrfToken, RedirectUrl};
use thiserror::Error;
use url::Url;

use crate::{
    contracts::repositories::music_account_provider_repository::{MusicAccountProviderRepository, MusicAccountProviderRepositoryError},
    value_objects::provider::provider_id::ProviderId,
};

pub struct OAuth2AuthorizeUserQueryResponse {
    pub auth_url: Url,
}

#[derive(Debug, Error)]
pub enum OAuth2AuthorizeUserQueryError {
    #[error("Provider not found")]
    ProviderNotFound,
    #[error("InternalError: {0}")]
    InternalError(String),
}

impl From<MusicAccountProviderRepositoryError> for OAuth2AuthorizeUserQueryError {
    fn from(error: MusicAccountProviderRepositoryError) -> Self {
        Self::InternalError(error.to_string())
    }
}

pub struct OAuth2AuthorizeUserQuery {
    provider_id: ProviderId,
    redirect_url: RedirectUrl,
}

impl OAuth2AuthorizeUserQuery {
    pub fn new(
        provider_id: ProviderId,
        redirect_url: RedirectUrl,
    ) -> Self {
        Self {
            provider_id,
            redirect_url,
        }
    }

    pub async fn execute(
        self,
        auth_repo: &impl MusicAccountProviderRepository,
    ) -> Result<OAuth2AuthorizeUserQueryResponse, OAuth2AuthorizeUserQueryError> {
        let (client_id, client_secret) = match self.provider_id.as_str() {
            "spotify" => (
                ClientId::new(std::env::var("SPOTIFY_OAUTH2_CLIENT_ID").unwrap()),
                ClientSecret::new(std::env::var("SPOTIFY_OAUTH2_CLIENT_SECRET").unwrap()),
            ),
            "deezer" => (
                ClientId::new(std::env::var("DEEZER_OAUTH2_CLIENT_ID").unwrap()),
                ClientSecret::new(std::env::var("DEEZER_OAUTH2_CLIENT_SECRET").unwrap()),
            ),
            _ => return Err(OAuth2AuthorizeUserQueryError::ProviderNotFound),
        };

        let Some(auth_provider) = auth_repo.get(&self.provider_id).await? else {
            return Err(OAuth2AuthorizeUserQueryError::ProviderNotFound);
        };

        if !auth_provider.authentication_allowed {
            return Err(OAuth2AuthorizeUserQueryError::ProviderNotFound);
        }

        // Create oauth2 client
        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_provider.auth_url)
            .set_redirect_uri(self.redirect_url);

        // Generate authorization URL
        let (auth_url, _) = client
            .authorize_url(custom_csrf_token(auth_provider.id.value()))
            .add_scopes(auth_provider.authorizations_needed)
            .url();

        Ok(OAuth2AuthorizeUserQueryResponse { auth_url })
    }
}

// Csrf token custom function enabling get information about-provider
fn custom_csrf_token(provider_id: String) -> impl FnOnce() -> CsrfToken {
    move || {
        CsrfToken::new(format!(
            "{}-{}",
            provider_id,
            CsrfToken::new_random().secret()
        ))
    }
}
