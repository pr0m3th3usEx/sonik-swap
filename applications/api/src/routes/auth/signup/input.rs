use serde::Deserialize;
use snk_core::value_objects::{misc::email::Email, user::user_password::UserPassword};

#[derive(Debug, Deserialize)]
pub struct CredentialsSignupBody {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct CredentialsSignupRequest {
    pub(super) email: Email,
    pub(super) password: UserPassword,
}

impl TryFrom<CredentialsSignupBody> for CredentialsSignupRequest {
    type Error = String;

    fn try_from(body: CredentialsSignupBody) -> Result<Self, Self::Error> {
        let email = Email::new(body.email)
            .map_err(|err| {
                tracing::error!({ %err }, "Error while parsing email");
                // Field name as error
                "email".to_string()
            })?;
        let password = UserPassword::new(body.password)
            .map_err(|err| {
                tracing::error!({ %err }, "Error while parsing password");
                // Field name as error
                "password".to_string()
            })?;

        Ok(Self {
            email,
            password
        })
    }
}