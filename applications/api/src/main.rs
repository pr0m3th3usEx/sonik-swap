use axum::{routing::get, Router};
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

    // TODO API routes
    //
    // - GET  /auth/{providerId}: Return OAuth2 authentication URL + ?=state=providerId
    // - GET  /auth/callback: FindOrCreate user account
    //
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

    let app = Router::new().route("/", get(health));

    // TODO import from environment (PORT, HOST)
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
