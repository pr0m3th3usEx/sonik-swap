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

    async fn update(&self, user: User) -> UserRepositoryResult<User>;

    async fn get(&self, user_id: Uuid) -> UserRepositoryResult<Option<User>>;

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>>;

    async fn delete(&self, user: User) -> UserRepositoryResult<User>;
}
