use std::marker::PhantomData;

use axum::{
    async_trait, body::Body, extract::{FromRequest, Request}, http::{Response, StatusCode}, response::IntoResponse, Json, RequestExt
};
use serde_json::json;
use tracing::error;

pub enum BodyParsingError {
    InvalidField(String),
    InvalidBody,
}

impl IntoResponse for BodyParsingError {
    fn into_response(self) -> axum::response::Response {
        let mut response = Response::new(Body::from(match self {
            BodyParsingError::InvalidField(field) => json!({ "error": "field", "field": field }).to_string(),
            BodyParsingError::InvalidBody => json!({ "error": "body" }).to_string(),
        }));

        *response.status_mut() = StatusCode::BAD_REQUEST;

        response
    }
}

pub struct AppJsonBody<T, U>(pub U, pub PhantomData<T>)
where
    U: TryFrom<T>;

#[async_trait]
impl<S, T, U> FromRequest<S> for AppJsonBody<T, U>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    <Json<T> as FromRequest<()>>::Rejection: std::fmt::Debug,
    U: TryFrom<T>,
    <U as TryFrom<T>>::Error: std::fmt::Display,
    T: 'static,
{
    type Rejection = BodyParsingError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = req.extract::<Json<T>, _>().await.map_err(|e| {
            error!({ ?e }, "Error while extracting body");
            BodyParsingError::InvalidBody
        })?;

        Ok(Self(U::try_from(payload).map_err(|e| {
            error!({ "field" = %e }, "Error validating body");
            BodyParsingError::InvalidField(e.to_string())
        })?, PhantomData))
    }
}
