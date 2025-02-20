use thiserror::Error;

use crate::entities::provider_account::ProviderAccount;

#[derive(Debug, Error)]
pub enum ProviderAccountRepositoryError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}

pub type ProviderAccountRepositoryResult<T> = Result<T, ProviderAccountRepositoryError>;

pub trait ProviderAccountRepository {
    async fn get_logged_user(&self) -> ProviderAccountRepositoryResult<ProviderAccount>;
}
