mod routes;
mod state;
pub mod utils;

use adapters::{
    in_memory::{
        music_account_provider_repository::InMemoryMusicAccountProviderRepository,
        user_repository::InMemoryUserRepository,
    },
    misc::{
        jwt_provider::JwtProvider, password_provider_prod::PasswordProviderProd,
        user_id_provider_prod::UserIdProviderProd,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use routes::auth::{login, signup};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,snk_core=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Check env vars
    let _ = std::env::var("SPOTIFY_OAUTH2_CLIENT_ID").expect("SPOTIFY_OAUTH2_CLIENT_ID set");
    let _ =
        std::env::var("SPOTIFY_OAUTH2_CLIENT_SECRET").expect("SPOTIFY_OAUTH2_CLIENT_SECRET set");
    let _ = std::env::var("APP_DASHBOARD_URL").expect("APP_DASHBOARD_URL set");
    let _ = std::env::var("SPOTIFY_OAUTH2_BASE_URL").expect("SPOTIFY_OAUTH2_BASE_URL set");

    // Initialize repositories & providers
    let user_repository = InMemoryUserRepository::default();
    let user_id_provider = UserIdProviderProd::default();
    let password_provider = PasswordProviderProd::default();
    let access_token_provider = JwtProvider::new(
        std::env::var("ACCESS_TOKEN_SECRET")
            .expect("ACCESS_TOKEN_SECRET set")
            .to_string(),
    );
    let refresh_token_provider = JwtProvider::new(
        std::env::var("REFRESH_TOKEN_SECRET")
            .expect("REFRESH_TOKEN_SECRET set")
            .to_string(),
    );

    let music_account_provider_repo = InMemoryMusicAccountProviderRepository::seed();

    // TODO API routes

    // - GET  /user/me: Current logged in user information
    // - POST /user/tracks: Add tracks in user liked/favourite tracks
    // - DELETE /user/tracks: Delete tracks in user liked/favourite tracks
    //
    // - POST /providers: Init permission request for a user 3rd party account
    // - GET  /providers/grant: OAuth2 callback for permissions granting
    // - GET  /providers/status: Get connection status of user's providers
    // - DELETE /providers/revoke: Revoke provider access
    //
    // - GET    /providers/{providerType}/playlists : Get playlists
    // - POST   /providers/{providerType}/playlists : Create playlist
    // - DELETE /providers/{providerType}/playlists/{playlistId} : Delete playlist
    // - GET    /providers/{providerType}/playlists/{playlistId} : Fetch playlist info
    // - GET    /providers/{providerType}/playlists/{playlistId}/tracks : Fetch tracks of the playlist
    // - POST   /providers/{providerType}/playlist/{playlistId}/tracks : Add tracks to playlist
    // - DELETE /providers/{providerType}/playlist/{playlistId}/tracks : Delete tracks from playlist

    let auth_routes = Router::new()
        .route("/signup", post(signup::handler::<_, _, _, _, _, _>))
        // .route("/signup/oauth2/{provider_id}", get(signup::oauth2::handler::<_, _, _, _, _,>))
        // .route("/signup/oauth2/{provider_id}/callback", post(signup::oauth2::handler::<_, _, _, _, _,>))
        .route("/login", post(login::handler::<_, _, _, _, _, _>))
        .route(
            "/login/oauth2/:provider_id/callback",
            get(login::oauth2::callback::handler),
        )
        .route("/login/oauth2/:provider_id", get(login::oauth2::handler));

    let app = Router::new()
        .route("/api", get(health))
        .nest("/api/auth", auth_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state::AppState {
            user_repo: user_repository,
            user_id_provider,
            password_provider,
            access_token_provider,
            refresh_token_provider,
            music_account_provider_repo,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("TCPListener: Could not bind port");

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Axum: could not run server");
}

async fn health() -> &'static str {
    "OK"
}
