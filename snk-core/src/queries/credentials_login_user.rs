use chrono::{Duration, Utc};
use thiserror::Error;

use crate::{
    contracts::{
        providers::{
            password_provider::{PasswordProvider, PasswordProviderError},
            token_provider::{TokenProvider, TokenProviderError},
        },
        repositories::user_repository::{UserRepository, UserRepositoryError},
    },
    value_objects::{auth::auth_token_claims::AuthTokenClaims, misc::email::Email, user::user_password::UserPassword},
};

pub struct CredentialsLoginUserQuery {
    email: Email,
    password: UserPassword,
}

#[derive(Debug, Error)]
pub enum CredentialsLoginUserQueryError {
    #[error("Email or password incorrect")]
    BadCredentials,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("InternalError {0}")]
    InternalError(String),
}

impl From<PasswordProviderError> for CredentialsLoginUserQueryError {
    fn from(error: PasswordProviderError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<TokenProviderError> for CredentialsLoginUserQueryError {
    fn from(error: TokenProviderError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<UserRepositoryError> for CredentialsLoginUserQueryError {
    fn from(error: UserRepositoryError) -> Self {
        Self::InternalError(error.to_string())
    }
}

pub struct CredentialsLoginUserQueryOutput {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Duration,
}

impl CredentialsLoginUserQuery {
    pub fn new(email: Email, password: UserPassword) -> Self {
        Self { email, password }
    }

    pub async fn execute(
        self,
        user_repo: &impl UserRepository,
        password_provider: &impl PasswordProvider,
        access_token_provider: &impl TokenProvider,
        refresh_token_provider: &impl TokenProvider,
    ) -> Result<CredentialsLoginUserQueryOutput, CredentialsLoginUserQueryError> {
        // Verify if user exists
        let maybe_user = user_repo.get_from_email(&self.email).await?;
        let Some(user) = maybe_user else {
          return Err(CredentialsLoginUserQueryError::BadCredentials);
        };

        // Check password
        if !password_provider.verify_password(self.password.as_ref(), user.password.as_ref()).await? { 
          return Err(CredentialsLoginUserQueryError::BadCredentials);
        }

        // Check email verified
        // if !user.email_verified {
        //     // TODO trigger sending verification email via event bus
        //     return Err(CredentialsLoginUserQueryError::EmailNotVerified);
        // }

        // Grant access & refresh token
        let access_token_exp = Utc::now() + ACCESS_TOKEN_EXP_TIME;
        let refresh_token_exp = Utc::now() + REFRESH_TOKEN_EXP_TIME;

        let access_token_claims = AuthTokenClaims::new(
            user.id.value().to_string(),
            access_token_exp.timestamp(),
            0, // TODO once integration
        );

        let refresh_token_claims = AuthTokenClaims::new(
            user.id.value().to_string(),
            refresh_token_exp.timestamp(),
            0, // TODO once integration
        );

        let access_token = access_token_provider.generate_token(access_token_claims).await?;
        let refresh_token = refresh_token_provider.generate_token(refresh_token_claims).await?;

        Ok(CredentialsLoginUserQueryOutput {
            access_token,
            refresh_token,
            expires_in: ACCESS_TOKEN_EXP_TIME,
        })
    }
}

const ACCESS_TOKEN_EXP_TIME: Duration = Duration::days(1);
const REFRESH_TOKEN_EXP_TIME: Duration = Duration::days(7);