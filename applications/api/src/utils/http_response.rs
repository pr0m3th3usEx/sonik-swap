use axum::{body::Body, response::IntoResponse};

pub struct EmptyResponse();

impl IntoResponse for EmptyResponse {
    fn into_response(self) -> axum::response::Response {
        Body::from(()).into_response()
    }
}
