use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use snk_core::contracts::repositories::user_repository::UserRepository;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CredentialsLoginRequest {
  email: String,
  password: String,
}

/// - POST /auth/login: Login user
pub async fn handler<UserRepo>(
  State(_state): State<AppState<UserRepo>>,
  Json(payload): Json<CredentialsLoginRequest>,
) -> impl IntoResponse
where
  UserRepo: UserRepository,
{
  tracing::info!("{:?}", payload);
  "login".to_string()
}
