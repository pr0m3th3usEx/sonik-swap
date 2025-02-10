use thiserror::Error;
use uuid::Uuid;

use crate::entities::user::User;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}
pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

/// Repository managing user accounts
pub trait UserRepository {
    async fn add(&self, user: User) -> UserRepositoryResult<User>;

    async fn update(&self, old: User, new: User) -> UserRepositoryResult<User>;

    async fn get(&self, user_id: Uuid) -> UserRepositoryResult<Option<User>>;

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>>;

    async fn delete(&self, user_id: Uuid) -> UserRepositoryResult<User>;
}

// Database implementation of users
// USER
// id: String PRIMARY KEY,
// username: string
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
