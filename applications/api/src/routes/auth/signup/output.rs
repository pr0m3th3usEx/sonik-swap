use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};
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
        let mut response = Body::from(serde_json::to_string(&self).unwrap()).into_response();

        *response.status_mut() = StatusCode::from_u16(self.status).expect("invalid status code");

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