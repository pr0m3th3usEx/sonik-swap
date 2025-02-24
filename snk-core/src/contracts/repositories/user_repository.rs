use thiserror::Error;

use crate::{
    entities::{provider_account::ProviderAccount, user::User},
    value_objects::{misc::email::Email, provider::provider_id::ProviderId, user::user_id::UserId},
};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}
pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

/// Repository managing user accounts
pub trait UserRepository: Send + Sync {
    async fn add(&self, user: User) -> UserRepositoryResult<User>;

    async fn update(&self, old: User, new: User) -> UserRepositoryResult<User>;

    async fn get(&self, user_id: &UserId) -> UserRepositoryResult<Option<User>>;

    async fn get_user_provider_account_id(
        &self,
        provider_id: &ProviderId,
        external_account_info: &ProviderAccount,
    ) -> UserRepositoryResult<Option<User>>;

    async fn get_from_email(&self, email: &Email) -> UserRepositoryResult<Option<User>>;

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>>;

    async fn delete(&self, user_id: &UserId) -> UserRepositoryResult<User>;
}

// Database implementation of users
// USER
// id: String PRIMARY KEY,
// email: string
// email_verified: boolean
// password: string
// created_at: timestamp
// updated_at: timestamp
//
// EMAIL_VERIFICATION
// user_id String
// token: String UNIQUE KEY
// consumed: boolean
// expires_at: timestamp
// created_at: timestamp
