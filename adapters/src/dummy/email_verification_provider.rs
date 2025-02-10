use chrono::DateTime;
use snk_core::{
    contracts::repositories::email_verification_repository::{
        EmailVerificationRepository, EmailVerificationRepositoryResult,
    },
    entities::email_verification::EmailVerificationToken,
};

#[derive(Default)]
pub struct DummyEmailVerificationRepository {}

impl EmailVerificationRepository for DummyEmailVerificationRepository {
    async fn add(
        &self,
        ev_token: EmailVerificationToken,
    ) -> EmailVerificationRepositoryResult<EmailVerificationToken> {
        Ok(ev_token)
    }

    async fn update(
        &self,
        _old: EmailVerificationToken,
        new: EmailVerificationToken,
    ) -> EmailVerificationRepositoryResult<EmailVerificationToken> {
        Ok(new)
    }

    async fn get(
        &self,
        user_id: &str,
        token: &str,
    ) -> EmailVerificationRepositoryResult<Option<EmailVerificationToken>> {
        Ok(Some(EmailVerificationToken {
            user_id: user_id.to_string(),
            token: token.to_string(),
            consumed: false,
            expires_at: DateTime::from(std::time::SystemTime::now()),
            created_at: DateTime::from(std::time::SystemTime::now()),
        }))
    }
}
