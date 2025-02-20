use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use oauth2::{AuthUrl, Scope, TokenUrl};
use snk_core::{
    contracts::repositories::music_account_provider_repository::{
        MusicAccountProviderRepository, MusicAccountProviderRepositoryResult,
    },
    entities::music_account_provider::MusicAccountProvider,
    value_objects::provider::{provider_id::ProviderId, provider_name::ProviderName},
};

#[derive(Default, Clone)]
pub struct InMemoryMusicAccountProviderRepository {
    providers: Arc<RwLock<HashMap<ProviderId, MusicAccountProvider>>>,
}

impl InMemoryMusicAccountProviderRepository {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn seed() -> Self {
        let repo: Self = Default::default();

        repo.providers.write().expect("lock poisoned").insert(
            ProviderId::new("spotify".to_string()),
            MusicAccountProvider::new(
                ProviderId::new("spotify".to_string()),
                ProviderName::new("spotify"),
                0x2cdc6a,
                AuthUrl::new("https://accounts.spotify.com/authorize".to_string()).unwrap(),
                TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).unwrap(),
                true,
                vec![
                    Scope::new(String::from("user-library-read")),
                    Scope::new(String::from("user-library-modify")),
                    Scope::new(String::from("playlist-modify-private")),
                    Scope::new(String::from("playlist-read-private")),
                    Scope::new(String::from("user-read-email")),
                    Scope::new(String::from("user-read-private")),
                ],
            ),
        );

        repo
    }
}

impl MusicAccountProviderRepository for InMemoryMusicAccountProviderRepository {
    async fn get(
        &self,
        id: &ProviderId,
    ) -> MusicAccountProviderRepositoryResult<Option<MusicAccountProvider>> {
        let store = self.providers.read().expect("lock poisoned");

        Ok(store.get(id).cloned())
    }

    async fn get_all(&self) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>> {
        let store = self.providers.read().expect("lock poisoned");

        Ok(Vec::from_iter(store.clone().into_values()))
    }

    async fn add(
        &self,
        provider: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider> {
        let mut store = self.providers.write().expect("lock poisoned");

        store.insert(provider.id.clone(), provider.clone());

        Ok(provider)
    }

    async fn update(
        &self,
        new: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider> {
        let mut store = self.providers.write().expect("lock poisoned");

        store.entry(new.id.clone()).and_modify(|e| *e = new.clone());

        Ok(new)
    }

    async fn get_auth_providers(
        &self,
    ) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>> {
        let store = self.providers.read().expect("lock poisoned");

        let music_providers = store
            .iter()
            .map(|(_, music_provider)| music_provider)
            .filter(|music_provider| music_provider.authentication_allowed)
            .cloned()
            .collect();

        Ok(music_providers)
    }
}
