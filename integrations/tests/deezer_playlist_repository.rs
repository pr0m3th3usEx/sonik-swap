use integrations::deezer::DeezerPlaylistRepository;
use oauth2::{AuthUrl, Scope, TokenUrl};
use snk_core::{
    contracts::repositories::playlist_repository::PlaylistRepository,
    entities::music_account_provider::MusicAccountProvider,
    value_objects::{
        playlist_id::PlaylistId,
        provider::{provider_id::ProviderId, provider_name::ProviderName},
    },
};

#[tokio::test]
async fn test_get_playlist_no_auth() {
    // Init repository
    let music_account_provider = MusicAccountProvider::new(
        ProviderId::new("deezer".to_string()),
        ProviderName::new("Deezer"),
        0,
        AuthUrl::new("https://connect.deezer.com/oauth/auth.php".to_string()).expect("valid url"),
        TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).expect("valid url"),
        true,
        vec![
            Scope::new("manage_library".to_string()),
            Scope::new("basic_access".to_string()),
        ],
    );
    let playlist_repo = DeezerPlaylistRepository::new(&music_account_provider, "".to_string())
        .expect("repo initialized");

    // Call api
    let playlist_id = PlaylistId::Owned("908622995".to_string());

    let result = playlist_repo.get(&playlist_id).await;

    assert!(result.is_err());
}
