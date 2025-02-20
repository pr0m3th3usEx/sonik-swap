use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordProviderError {
    #[error("Could not hash password: {0}")]
    HashError(String),
    #[error("Could not verify password: {0}")]
    VerifyError(String),
}

pub type PasswordProviderResult<T> = Result<T, PasswordProviderError>;

pub trait PasswordProvider {
    async fn hash_password(&self, password: &str) -> PasswordProviderResult<String>;
    async fn verify_password(&self, password: &str, hash: &str) -> PasswordProviderResult<bool>;
}
