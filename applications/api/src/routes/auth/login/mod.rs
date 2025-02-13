mod input;
mod output;

use axum::{extract::State, response::IntoResponse};
use input::{CredentialsLoginBody, CredentialsLoginRequest};
use snk_core::contracts::{providers::{password_provider::PasswordProvider, user_id_provider::UserIdProvider}, repositories::user_repository::UserRepository};

use crate::{state::AppState, utils::extractors::body::AppJsonBody};

/// - POST /auth/login: Login user
pub async fn handler<UserRepo, UserIdProv, PassswordProv>(
    State(_state): State<AppState<UserRepo, UserIdProv, PassswordProv>>,
    AppJsonBody(payload, _): AppJsonBody<CredentialsLoginBody, CredentialsLoginRequest>,
) -> impl IntoResponse
where
    UserRepo: UserRepository,
    UserIdProv: UserIdProvider,
    PassswordProv: PasswordProvider,
{
    tracing::info!("{:?}", payload);
    "login".to_string()
}
