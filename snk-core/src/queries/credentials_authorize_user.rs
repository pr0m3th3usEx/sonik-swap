use thiserror::Error;

use crate::{
    contracts::{
        providers::{
            password_provider::{PasswordProvider, PasswordProviderError},
            token_provider::{TokenProvider, TokenProviderError},
        },
        repositories::user_repository::{UserRepository, UserRepositoryError},
    },
    utils::auth::generate_token_pair,
    value_objects::{misc::email::Email, user::user_password::UserPassword},
};

use super::LoginUserQueryOutput;

pub struct CredentialsAuthorizeUserQuery {
    email: Email,
    password: UserPassword,
}

#[derive(Debug, Error)]
pub enum CredentialsAuthorizeUserQueryError {
    #[error("Email or password incorrect")]
    BadCredentials,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("InternalError {0}")]
    InternalError(String),
}

impl From<PasswordProviderError> for CredentialsAuthorizeUserQueryError {
    fn from(error: PasswordProviderError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<TokenProviderError> for CredentialsAuthorizeUserQueryError {
    fn from(error: TokenProviderError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<UserRepositoryError> for CredentialsAuthorizeUserQueryError {
    fn from(error: UserRepositoryError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl CredentialsAuthorizeUserQuery {
    pub fn new(email: Email, password: UserPassword) -> Self {
        Self { email, password }
    }

    pub async fn execute(
        self,
        user_repo: &impl UserRepository,
        password_provider: &impl PasswordProvider,
        access_token_provider: &impl TokenProvider,
        refresh_token_provider: &impl TokenProvider,
    ) -> Result<LoginUserQueryOutput, CredentialsAuthorizeUserQueryError> {
        // Verify if user exists
        let maybe_user = user_repo.get_from_email(&self.email).await?;
        let Some(user) = maybe_user else {
            return Err(CredentialsAuthorizeUserQueryError::BadCredentials);
        };

        // Check password
        if !password_provider
            .verify_password(self.password.as_ref(), user.password.as_ref())
            .await?
        {
            return Err(CredentialsAuthorizeUserQueryError::BadCredentials);
        }

        // Check email verified
        if !user.email_verified {
            // TODO trigger sending verification email via event bus
            return Err(CredentialsAuthorizeUserQueryError::EmailNotVerified);
        }

        // Grant access & refresh token
        Ok(
            generate_token_pair(user.id(), access_token_provider, refresh_token_provider)
                .await
                .map(
                    |(access_token, refresh_token, expires_in)| LoginUserQueryOutput {
                        access_token,
                        refresh_token,
                        expires_in,
                    },
                )?,
        )
    }
}
