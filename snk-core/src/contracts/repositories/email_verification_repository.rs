use thiserror::Error;

use crate::entities::auth::email_verification::EmailVerificationToken;

#[derive(Debug, Error)]
pub enum EmailVerificationRepositoryError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}

pub type EmailVerificationRepositoryResult<T> = Result<T, EmailVerificationRepositoryError>;

pub trait EmailVerificationRepository: Send + Sync {
    async fn add(
        &self,
        ev_token: EmailVerificationToken,
    ) -> EmailVerificationRepositoryResult<EmailVerificationToken>;
    async fn update(
        &self,
        old: EmailVerificationToken,
        new: EmailVerificationToken,
    ) -> EmailVerificationRepositoryResult<EmailVerificationToken>;
    async fn get(
        &self,
        user_id: &str,
        token: &str,
    ) -> EmailVerificationRepositoryResult<Option<EmailVerificationToken>>;
}
