use chrono::Duration;

pub mod credentials_authorize_user;
pub mod oauth2_authorize_user;
pub mod oauth2_login_callback;

pub struct LoginUserQueryOutput {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Duration,
}
