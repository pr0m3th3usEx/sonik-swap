mod output;
mod input;

use adapters::misc::{password_provider_prod::PasswordProviderProd, user_id_provider_prod::UserIdProviderProd};
use axum::{extract::State, http::StatusCode};
use input::{CredentialsSignupBody, CredentialsSignupRequest};
use output::{SignupError, SignupResponse};
use snk_core::{commands::create_credentials_user::{CreateCredentialsUserCommand, CreateCredentialsUserCommandError}, contracts::repositories::user_repository::UserRepository};

use crate::{state::AppState, utils::extractors::body::AppJsonBody};

impl From<CreateCredentialsUserCommandError> for SignupError {
  fn from(error: CreateCredentialsUserCommandError) -> Self {
      match error {
          CreateCredentialsUserCommandError::EmailAlreadyExists => Self {
              status: StatusCode::CONFLICT.as_u16(),
              message: CreateCredentialsUserCommandError::EmailAlreadyExists.to_string(),
          },
          CreateCredentialsUserCommandError::InternalError(_) => Self {
              status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
              message: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
          },
      }
  }
}

/// - POST /auth/signup: Create a new user account
pub async fn handler<UserRepo>(
  State(state): State<AppState<UserRepo>>,
  AppJsonBody(payload, _): AppJsonBody<CredentialsSignupBody, CredentialsSignupRequest>,
) -> Result<SignupResponse, SignupError>
where
  UserRepo: UserRepository,
{
  let user_id_provider = UserIdProviderProd::new();
  let password_provider = PasswordProviderProd::new(None);

  CreateCredentialsUserCommand::new(payload.email, payload.password)
      .execute(&state.user_repo, &user_id_provider, &password_provider)
      .await?;

  Ok(SignupResponse::default())
}