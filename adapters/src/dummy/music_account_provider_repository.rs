use oauth2::{AuthUrl, Scope, TokenUrl};
use snk_core::{
    contracts::repositories::music_account_provider_repository::{
        MusicAccountProviderRepository, MusicAccountProviderRepositoryResult,
    },
    entities::music_account_provider::MusicAccountProvider,
    value_objects::provider::{provider_id::ProviderId, provider_name::ProviderName},
};

pub struct DummyMusicAccountProviderRepository {}

impl MusicAccountProviderRepository for DummyMusicAccountProviderRepository {
    async fn get(
        &self,
        id: &ProviderId,
    ) -> MusicAccountProviderRepositoryResult<Option<MusicAccountProvider>> {
        Ok(Some(MusicAccountProvider::new(
            id.clone(),
            ProviderName::new("spotify"),
            0x2cdc6a,
            AuthUrl::new("https://accounts.spotify.com/authorize".to_string()).unwrap(),
            TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).unwrap(),
            true,
            vec![Scope::new(String::from("manage_library"))],
        )))
    }

    async fn get_all(&self) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>> {
        Ok(vec![MusicAccountProvider::new(
            ProviderId::new(String::from("spotify")),
            ProviderName::new("spotify"),
            0x2cdc6a,
            AuthUrl::new("https://accounts.spotify.com/authorize".to_string()).unwrap(),
            TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).unwrap(),
            true,
            vec![Scope::new(String::from("manage_library"))],
        )])
    }

    async fn add(
        &self,
        provider: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider> {
        Ok(provider)
    }

    async fn update(
        &self,
        new: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider> {
        Ok(new)
    }

    async fn get_auth_providers(
        &self,
    ) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>> {
        Ok(vec![MusicAccountProvider::new(
            ProviderId::new(String::from("spotify")),
            ProviderName::new("spotify"),
            0x2cdc6a,
            AuthUrl::new("https://accounts.spotify.com/authorize".to_string()).unwrap(),
            TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).unwrap(),
            true,
            vec![Scope::new(String::from("manage_library"))],
        )])
    }
}
