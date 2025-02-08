use snk_core::{
    contracts::repositories::music_account_provider_repository::{
        MusicAccountProviderRepository, MusicAccountProviderRepositoryResult,
    },
    entities::music_account_provider::MusicAccountProvider,
    value_objects::provider::provider_id::ProviderId,
};
use uuid::Uuid;

pub struct DummyMusicAccountProviderRepository {}

impl MusicAccountProviderRepository for DummyMusicAccountProviderRepository {
    async fn get(
        &self,
        _id: Uuid,
    ) -> MusicAccountProviderRepositoryResult<Option<MusicAccountProvider>> {
        Ok(Some(MusicAccountProvider::new(
            ProviderId::new(String::from("spotify")),
            String::from("spotify"),
            String::from("#2cdc6a"),
            "https://accounts.spotify.com/authorize".parse().unwrap(),
            "https://accounts.spotify.com/api/token".parse().unwrap(),
            vec![String::from("manage_library")],
        )))
    }

    async fn get_all(&self) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>> {
        Ok(vec![MusicAccountProvider::new(
            ProviderId::new(String::from("spotify")),
            String::from("spotify"),
            String::from("#2cdc6a"),
            "https://accounts.spotify.com/authorize".parse().unwrap(),
            "https://accounts.spotify.com/api/token".parse().unwrap(),
            vec![String::from("manage_library")],
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
}
