use integrations::deezer::DeezerPlaylistRepository;
use snk_core::{
    contracts::repositories::playlist_repository::PlaylistRepository,
    entities::music_account_provider::MusicAccountProvider,
    value_objects::{playlist_id::PlaylistId, provider::provider_id::ProviderId},
};

#[tokio::test]
async fn test_get_playlist_no_auth() {
    // Init repository
    let music_account_provider = MusicAccountProvider::new(
        ProviderId::new("deezer".to_string()),
        "Deezer".to_string(),
        "#000000".to_string(),
        "https://connect.deezer.com/oauth/auth.php"
            .parse()
            .expect("valid url"),
        "https://accounts.spotify.com/api/token"
            .parse()
            .expect("valid url"),
        vec!["manage_library".to_string(), "basic_access".to_string()],
    );
    let playlist_repo = DeezerPlaylistRepository::new(&music_account_provider, "".to_string())
        .expect("repo initialized");

    // Call api
    let playlist_id = PlaylistId::Owned("908622995".to_string());

    let result = playlist_repo.get(&playlist_id).await;

    assert!(result.is_err());
}
