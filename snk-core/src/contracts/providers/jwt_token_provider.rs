use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenProviderError {
    #[error("InternalError: {0}")]
    InternalError(String),
}

pub type TokenProviderResult<T> = Result<T, TokenProviderError>;

pub trait TokenProvider {
    async fn generate_token<T>(
        &self,
        claims: T,
        expiration: DateTime<Utc>,
        secret: String,
    ) -> TokenProviderResult<String>
    where
        T: serde::Serialize;

    async fn verify_token<T>(&self, token: &str, secret: String) -> TokenProviderResult<T>
    where
        T: serde::de::DeserializeOwned;
}
