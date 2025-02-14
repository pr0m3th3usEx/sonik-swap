use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope};

pub enum OAuth2Error {
    NotFound,
    InternalError,
}

impl IntoResponse for OAuth2Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = Body::from(()).into_response();

        *(response.status_mut()) = match self {
            OAuth2Error::NotFound => StatusCode::NOT_FOUND,
            OAuth2Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        response
    }
}

// Csrf token custom function enabling get information about-provider
fn custom_csrf_token(provider_id: String) -> impl FnOnce() -> CsrfToken {
    move || {
        CsrfToken::new(format!(
            "{}-{}",
            provider_id,
            CsrfToken::new_random().secret()
        ))
    }
}

pub async fn handler(Path(provider_id): Path<String>) -> Result<Redirect, OAuth2Error> {
    let app_dashboard_url = std::env::var("APP_DASHBOARD_URL").unwrap();

    let client;
    let client_id: String;
    let client_secret: String;
    let auth_url: String;
    let redirect_url: String;

    // Hard coded allowed provider (spotify / deezer)
    // TODO Change later (not urgent)
    match provider_id.to_lowercase().as_str() {
        "spotify" => {
            // Check present of variables (verified at start up)
            let client_base_url = std::env::var("SPOTIFY_OAUTH2_BASE_URL").unwrap();

            client_id = std::env::var("SPOTIFY_OAUTH2_CLIENT_ID").unwrap();
            client_secret = std::env::var("SPOTIFY_OAUTH2_CLIENT_SECRET").unwrap();
            auth_url = format!("{}/authorize", client_base_url);
            redirect_url = format!("{}/auth/oauth2/callback", app_dashboard_url);

            client = BasicClient::new(ClientId::new(client_id))
                .set_client_secret(ClientSecret::new(client_secret))
                .set_auth_uri(AuthUrl::new(auth_url).map_err(|_| OAuth2Error::InternalError)?)
                .set_redirect_uri(
                    RedirectUrl::new(redirect_url).map_err(|_| OAuth2Error::InternalError)?,
                );

            let (auth_url, _) = client
                .authorize_url(custom_csrf_token(provider_id))
                .add_scope(Scope::new("user-library-read".to_string()))
                .add_scope(Scope::new("user-library-modify".to_string()))
                .add_scope(Scope::new("playlist-modify-private".to_string()))
                .add_scope(Scope::new("playlist-read-private".to_string()))
                .add_scope(Scope::new("user-read-private".to_string()))
                .add_scope(Scope::new("user-read-email".to_string()))
                .url();

            Ok(Redirect::temporary(auth_url.as_str()))
        }
        "deezer" => todo!(),
        _ => Err(OAuth2Error::NotFound),
    }
}
