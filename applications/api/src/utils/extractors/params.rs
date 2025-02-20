use std::marker::PhantomData;

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Path},
    http::{request::Parts, Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub enum ParamsParsingError {
    InvalidField(String),
    InvalidParams,
}

impl IntoResponse for ParamsParsingError {
    fn into_response(self) -> axum::response::Response {
        let mut response = Response::new(Body::from(match self {
            ParamsParsingError::InvalidField(field) => {
                json!({ "error": "field", "field": field }).to_string()
            }
            ParamsParsingError::InvalidParams => json!({ "error": "params" }).to_string(),
        }));

        *response.status_mut() = StatusCode::BAD_REQUEST;

        response
    }
}

pub struct AppPathParams<T, U>(pub U, pub PhantomData<T>);

#[async_trait]
impl<S, T, U> FromRequestParts<S> for AppPathParams<T, U>
where
    S: Send + Sync,
    Path<T>: FromRequestParts<S>,
    <Path<T> as FromRequestParts<S>>::Rejection: std::fmt::Debug,
    U: TryFrom<T>,
    <U as TryFrom<T>>::Error: std::fmt::Display,
    T: 'static,
{
    type Rejection = ParamsParsingError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(payload) = Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::error!({ ?e }, "Error while extracting params");
                ParamsParsingError::InvalidParams
            })?;

        Ok(Self(
            U::try_from(payload).map_err(|e| {
                tracing::error!({ "field" = %e }, "Error validating params");
                ParamsParsingError::InvalidField(e.to_string())
            })?,
            PhantomData,
        ))
    }
}
