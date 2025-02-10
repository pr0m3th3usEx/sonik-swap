mod routes;
mod state;

use adapters::in_memory::{
    email_verification_repository::InMemoryEmailVerificationRepository,
    user_repository::InMemoryUserRepository,
};
use axum::{
    routing::{get, post},
    Router,
};
use routes::auth::handlers as auth_handlers;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize repositories
    let user_repository = InMemoryUserRepository::default();
    let email_verification_repository = InMemoryEmailVerificationRepository::default();

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

    let auth_routes =
        Router::new()
            .route(
                "/api/auth/login",
                post(
                    auth_handlers::login::<
                        InMemoryUserRepository,
                        InMemoryEmailVerificationRepository,
                    >,
                ),
            )
            .route(
                "/api/auth/signup",
                post(
                    auth_handlers::signup::<
                        InMemoryUserRepository,
                        InMemoryEmailVerificationRepository,
                    >,
                ),
            );
    // .route(
    //     "/api/auth/oauth2",
    //     get(auth_handlers::oauth2::<
    //         InMemoryUserRepository,
    //         InMemoryEmailVerificationRepository,
    //     >),
    // )
    // .route(
    //     "/api/auth/oauth2/callback",
    //     get(auth_handlers::oauth2::<
    //         InMemoryUserRepository,
    //         InMemoryEmailVerificationRepository,
    //     >),
    // );

    let app = Router::new()
        .route("/api", get(health))
        .nest("/api", auth_routes)
        .with_state(state::AppState {
            user_repo: user_repository,
            email_verification_repo: email_verification_repository,
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
