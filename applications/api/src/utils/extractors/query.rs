use std::marker::PhantomData;

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Query},
    http::{request::Parts, Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub enum QueryParsingError {
    InvalidField(String),
    InvalidQuery,
}

impl IntoResponse for QueryParsingError {
    fn into_response(self) -> axum::response::Response {
        let mut response = Response::new(Body::from(match self {
            QueryParsingError::InvalidField(field) => {
                json!({ "error": "field", "field": field }).to_string()
            }
            QueryParsingError::InvalidQuery => json!({ "error": "query" }).to_string(),
        }));

        *response.status_mut() = StatusCode::BAD_REQUEST;

        response
    }
}

pub struct AppQueryParams<T, U>(pub U, pub PhantomData<T>);

#[async_trait]
impl<S, T, U> FromRequestParts<S> for AppQueryParams<T, U>
where
    S: Send + Sync,
    Query<T>: FromRequestParts<S>,
    <Query<T> as FromRequestParts<S>>::Rejection: std::fmt::Debug,
    U: TryFrom<T>,
    <U as TryFrom<T>>::Error: std::fmt::Display,
    T: 'static,
{
    type Rejection = QueryParsingError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(payload) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::error!({ ?e }, "Error while extracting body");
                QueryParsingError::InvalidQuery
            })?;

        Ok(Self(
            U::try_from(payload).map_err(|e| {
                tracing::error!({ "field" = %e }, "Error validating body");
                QueryParsingError::InvalidField(e.to_string())
            })?,
            PhantomData,
        ))
    }
}
