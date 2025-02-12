use thiserror::Error;
use tracing::info;

use crate::{
    contracts::{
        providers::{
            password_provider::{PasswordProvider, PasswordProviderError},
            user_id_provider::UserIdProvider,
        },
        repositories::user_repository::{UserRepository, UserRepositoryError},
    },
    entities::user::User,
};

pub struct CreateCredentialsUserCommand {
    email: String,
    password: String,
}

#[derive(Debug, Error)]
pub enum CreateCredentialsUserCommandError {
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<UserRepositoryError> for CreateCredentialsUserCommandError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::ServiceError(e) => {
                CreateCredentialsUserCommandError::InternalError(e)
            }
        }
    }
}

impl From<PasswordProviderError> for CreateCredentialsUserCommandError {
    fn from(error: PasswordProviderError) -> Self {
        match error {
            PasswordProviderError::HashError(e) => {
                CreateCredentialsUserCommandError::InternalError(e)
            }
            PasswordProviderError::VerifyError(e) => {
                CreateCredentialsUserCommandError::InternalError(e)
            }
        }
    }
}

impl CreateCredentialsUserCommand {
    pub async fn execute(
        self,
        user_repo: &impl UserRepository,
        id_provider: &impl UserIdProvider,
        password_provider: &impl PasswordProvider,
    ) -> Result<(), CreateCredentialsUserCommandError> {
        // Check if email already exists
        let maybe_user = user_repo.get_from_email(&self.email).await?.is_some();

        // If email already exists, return error
        if maybe_user {
            return Err(CreateCredentialsUserCommandError::EmailAlreadyExists);
        }

        // Hash password
        let hashed_password = password_provider.hash_password(&self.password).await?;

        // Add user to database

        let user_to_create = User::new(
            // TODO Replace by controlled ID in value objects
            id_provider.generate(),
            self.email,
            false,
            hashed_password,
            None,
            None,
        );

        let new_user = user_repo.add(user_to_create).await?;

        info!("User created: {:?}", new_user);

        // TODO Send email verification trigger to event bus

        Ok(())
    }
}
