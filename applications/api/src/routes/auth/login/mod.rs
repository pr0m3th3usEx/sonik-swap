mod input;
pub mod oauth2;
mod output;

use axum::{extract::State, http::StatusCode};
use input::{CredentialsLoginBody, CredentialsLoginRequest};
use output::{LoginError, LoginResponse};
use snk_core::{
    contracts::{
        providers::{
            password_provider::PasswordProvider, token_provider::TokenProvider,
            user_id_provider::UserIdProvider,
        },
        repositories::user_repository::UserRepository,
    },
    queries::credentials_authorize_user::{
        CredentialsAuthorizeUserQuery, CredentialsAuthorizeUserQueryError,
        CredentialsAuthorizeUserQueryOutput,
    },
};

use crate::{state::AppState, utils::extractors::body::AppJsonBody};

impl From<CredentialsAuthorizeUserQueryError> for LoginError {
    fn from(error: CredentialsAuthorizeUserQueryError) -> Self {
        tracing::error!({ %error }, "Error while executing query");
        match error {
            CredentialsAuthorizeUserQueryError::BadCredentials => Self {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: error.to_string(),
            },
            CredentialsAuthorizeUserQueryError::EmailNotVerified => Self {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: error.to_string(),
            },
            CredentialsAuthorizeUserQueryError::InternalError(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal error server".to_string(),
            },
        }
    }
}

impl From<CredentialsAuthorizeUserQueryOutput> for LoginResponse {
    fn from(output: CredentialsAuthorizeUserQueryOutput) -> Self {
        Self {
            access_token: output.access_token,
            refresh_token: output.refresh_token,
            expires_in: output.expires_in.num_seconds(),
        }
    }
}

/// - POST /auth/login: Login user
pub async fn handler<UserRepo, UserIdProv, PassswordProv, AccessTokenProv, RefreshTokenProv>(
    State(state): State<
        AppState<UserRepo, UserIdProv, PassswordProv, AccessTokenProv, RefreshTokenProv>,
    >,
    AppJsonBody(payload, _): AppJsonBody<CredentialsLoginBody, CredentialsLoginRequest>,
) -> Result<LoginResponse, LoginError>
where
    UserRepo: UserRepository,
    UserIdProv: UserIdProvider,
    PassswordProv: PasswordProvider,
    AccessTokenProv: TokenProvider,
    RefreshTokenProv: TokenProvider,
{
    let output = CredentialsAuthorizeUserQuery::new(payload.email, payload.password)
        .execute(
            &state.user_repo,
            &state.password_provider,
            &state.access_token_provider,
            &state.refresh_token_provider,
        )
        .await?;

    Ok(LoginResponse::from(output))
}
