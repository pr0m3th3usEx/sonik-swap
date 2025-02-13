use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};
use serde::Serialize;


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
  access_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {

}

impl IntoResponse for LoginResponse {
  fn into_response(self) -> axum::response::Response {
      let json = serde_json::to_string(&self);

      let mut response = Body::from(()).into_response();

      if let Ok(json) = json {
        *response.body_mut() = Body::from(json);
        *response.status_mut() = StatusCode::OK;
      } else {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
      }

      response
  }
}