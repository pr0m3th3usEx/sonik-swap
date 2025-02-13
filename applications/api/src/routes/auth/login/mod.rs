mod output;
mod input;

use axum::{extract::State, response::IntoResponse};
use input::{CredentialsLoginBody, CredentialsLoginRequest};
use snk_core::contracts::repositories::user_repository::UserRepository;

use crate::{state::AppState, utils::extractors::body::AppJsonBody};


/// - POST /auth/login: Login user
pub async fn handler<UserRepo>(
  State(_state): State<AppState<UserRepo>>,
  AppJsonBody(payload, _): AppJsonBody<CredentialsLoginBody, CredentialsLoginRequest>,
) -> impl IntoResponse
where
  UserRepo: UserRepository,
{
  tracing::info!("{:?}", payload);
  "login".to_string()
}
