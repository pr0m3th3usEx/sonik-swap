use serde::Deserialize;
use snk_core::value_objects::{misc::email::Email, user::user_password::UserPassword};

#[derive(Debug, Deserialize)]
pub struct CredentialsLoginBody {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct CredentialsLoginRequest {
    pub(super) email: Email,
    pub(super) password: UserPassword,
}

impl TryFrom<CredentialsLoginBody> for CredentialsLoginRequest {
    type Error = String;

    fn try_from(body: CredentialsLoginBody) -> Result<Self, Self::Error> {
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