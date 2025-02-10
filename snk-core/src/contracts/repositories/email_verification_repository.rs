use thiserror::Error;

use crate::entities::email_verification::EmailVerificationToken;

#[derive(Debug, Error)]
pub enum EmailVerificationRepositoryError {
  #[error("ServiceError: {0}")]
  ServiceError(String),    
}

pub type EmailVerificationRepositoryResult<T> = Result<T, EmailVerificationRepositoryError>;

pub trait EmailVerificationRepository {
    async fn add_email_verification(&self, user_id: &str, token: &str) -> EmailVerificationRepositoryResult<EmailVerificationToken>;
    async fn get_last_email_verification(&self, user_id: &str, token: &str) -> EmailVerificationRepositoryResult<Option<EmailVerificationToken>>;
}