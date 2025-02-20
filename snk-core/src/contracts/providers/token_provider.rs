use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenProviderError {
    #[error("ExpiredToken")]
    ExpiredToken,
    #[error("InvalidToken: {0}")]
    InvalidToken(String),
    #[error("InternalError: {0}")]
    InternalError(String),
}

pub type TokenProviderResult<T> = Result<T, TokenProviderError>;

pub trait TokenProvider {
    async fn generate_token<T>(&self, claims: T) -> TokenProviderResult<String>
    where
        T: serde::Serialize;

    async fn verify_token<T>(&self, token: &str) -> TokenProviderResult<T>
    where
        T: serde::de::DeserializeOwned + Default;
}
