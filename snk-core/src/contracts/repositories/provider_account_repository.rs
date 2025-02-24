use async_trait::async_trait;
use thiserror::Error;

use crate::entities::provider_account::ProviderAccount;

#[derive(Debug, Error)]
pub enum ProviderAccountRepositoryError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}

pub type ProviderAccountRepositoryResult<T> = Result<T, ProviderAccountRepositoryError>;

#[async_trait]
pub trait ProviderAccountRepository: Send + Sync {
    async fn get_logged_user(&self) -> ProviderAccountRepositoryResult<ProviderAccount>;
}
