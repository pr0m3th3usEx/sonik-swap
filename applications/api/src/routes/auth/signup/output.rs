use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SignupResponse {}

#[derive(Serialize)]
pub struct SignupError {
    pub(super) status: u16,
    pub(super) message: String,
}

impl IntoResponse for SignupError {
    fn into_response(self) -> axum::response::Response {
        let json = serde_json::to_string(&self);
        let mut response = Body::from(()).into_response();

        if let Ok(json) = json {
            *response.body_mut() = Body::from(json);
            *response.status_mut() =
                StatusCode::from_u16(self.status).expect("invalid status code");
        } else {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        }

        response
    }
}

impl IntoResponse for SignupResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = Response::new(Body::from(()));

        *response.status_mut() = StatusCode::CREATED;

        response
    }
}
