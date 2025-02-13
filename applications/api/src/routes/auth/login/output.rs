use axum::{body::Body, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub(super) access_token: String,
    pub(super) refresh_token: String,
    pub(super) expires_in: i64,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {
    pub(super) status: u16,
    pub(super) message: String,
}

impl IntoResponse for LoginError {
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
