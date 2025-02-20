pub mod callback;
mod input;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use input::{
    OAuth2AuthorizeRequestParams, OAuth2AuthorizeRequestParamsParsed, OAuth2AuthorizeRequestQuery,
    OAuth2AuthorizeRequestQueryParsed,
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
    queries::oauth2_authorize_user::{OAuth2AuthorizeUserQuery, OAuth2AuthorizeUserQueryError},
};

use crate::{
    state::AppState,
    utils::extractors::{params::AppPathParams, query::AppQueryParams},
};

pub enum OAuth2Error {
    NotFound,
    InternalError,
}

impl IntoResponse for OAuth2Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = Body::from(()).into_response();

        *(response.status_mut()) = match self {
            OAuth2Error::NotFound => StatusCode::NOT_FOUND,
            OAuth2Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        response
    }
}

impl From<OAuth2AuthorizeUserQueryError> for OAuth2Error {
    fn from(error: OAuth2AuthorizeUserQueryError) -> Self {
        match error {
            // TODO Improve error response
            OAuth2AuthorizeUserQueryError::ProviderNotFound => OAuth2Error::NotFound,
            OAuth2AuthorizeUserQueryError::InternalError(err) => {
                tracing::error!({ %err }, "Internal error");
                OAuth2Error::InternalError
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
    AppPathParams(params, _): AppPathParams<
        OAuth2AuthorizeRequestParams,
        OAuth2AuthorizeRequestParamsParsed,
    >,
    AppQueryParams(query, _): AppQueryParams<
        OAuth2AuthorizeRequestQuery,
        OAuth2AuthorizeRequestQueryParsed,
    >,
) -> Result<Redirect, OAuth2Error>
where
    UserRepo: UserRepository,
    MusicAccountProvRepo: MusicAccountProviderRepository,
    UserIdProv: UserIdProvider,
    PassswordProv: PasswordProvider,
    AccessTokenProv: TokenProvider,
    RefreshTokenProv: TokenProvider,
{
    let response = OAuth2AuthorizeUserQuery::new(params.provider_id, query.redirect_url)
        .execute(&state.music_account_provider_repo)
        .await?;

    Ok(Redirect::temporary(response.auth_url.as_str()))
}
