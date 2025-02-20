mod input;

use axum::{body::Body, extract::State, http::StatusCode, response::IntoResponse};
use input::{
    OAuth2CallbackRequestParams, OAuth2CallbackRequestParamsParsed, OAuth2CallbackRequestQuery,
    OAuth2CallbackRequestQueryParsed,
};
use snk_core::{
    contracts::{
        providers::{
            password_provider::PasswordProvider, token_provider::TokenProvider,
            user_id_provider::UserIdProvider,
        },
        repositories::{
            music_account_provider_repository::MusicAccountProviderRepository,
            user_repository::UserRepository,
        },
    },
    queries::oauth2_login_callback::{LoginOAuth2CallbackQuery, LoginOAuth2CallbackQueryError},
};

use crate::{
    routes::auth::login::output::LoginResponse,
    state::AppState,
    utils::extractors::{params::AppPathParams, query::AppQueryParams},
};

pub enum OAuth2LoginCallbackError {
    NotFound,
    BadRequest,
    InternalError,
}

impl IntoResponse for OAuth2LoginCallbackError {
    fn into_response(self) -> axum::response::Response {
        let mut response = Body::from(()).into_response();

        *(response.status_mut()) = match self {
            OAuth2LoginCallbackError::NotFound => StatusCode::NOT_FOUND,
            OAuth2LoginCallbackError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            OAuth2LoginCallbackError::BadRequest => StatusCode::BAD_REQUEST,
        };

        response
    }
}

impl From<LoginOAuth2CallbackQueryError> for OAuth2LoginCallbackError {
    fn from(error: LoginOAuth2CallbackQueryError) -> Self {
        match error {
            LoginOAuth2CallbackQueryError::ProviderNotFound => OAuth2LoginCallbackError::NotFound,
            LoginOAuth2CallbackQueryError::OAuth2AuthenticationUnvailable(_) => {
                OAuth2LoginCallbackError::BadRequest
            }
            LoginOAuth2CallbackQueryError::InternalError(_) => {
                OAuth2LoginCallbackError::InternalError
            }
        }
    }
}

pub async fn handler<
    UserRepo,
    MusicAccountProvRepo,
    UserIdProv,
    PassswordProv,
    AccessTokenProv,
    RefreshTokenProv,
>(
    State(state): State<
        AppState<
            UserRepo,
            MusicAccountProvRepo,
            UserIdProv,
            PassswordProv,
            AccessTokenProv,
            RefreshTokenProv,
        >,
    >,
    AppQueryParams(query, _): AppQueryParams<
        OAuth2CallbackRequestQuery,
        OAuth2CallbackRequestQueryParsed,
    >,
    AppPathParams(params, _): AppPathParams<
        OAuth2CallbackRequestParams,
        OAuth2CallbackRequestParamsParsed,
    >,
) -> Result</* LoginResponse */ (), OAuth2LoginCallbackError>
where
    UserRepo: UserRepository,
    MusicAccountProvRepo: MusicAccountProviderRepository,
    UserIdProv: UserIdProvider,
    PassswordProv: PasswordProvider,
    AccessTokenProv: TokenProvider,
    RefreshTokenProv: TokenProvider,
{
    tracing::debug!("EHHHEHEh");
    let response =
        LoginOAuth2CallbackQuery::new(params.provider_id, query.redirect_url, query.code)
            .execute(
                &state.user_repo,
                &state.music_account_provider_repo,
                &state.access_token_provider,
                &state.refresh_token_provider,
            )
            .await?;

    Ok(())
}
