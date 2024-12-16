use std::collections::HashSet;

use snk_core::{
    contracts::repositories::playlist_repository::{PlaylistRepository, PlaylistRepositoryResult},
    entities::playlist::{self, Playlist},
    value_objects::{playlist_id::PlaylistId, provider::provider_id::ProviderId},
};

pub struct DummyPlaylistRepository {}

impl PlaylistRepository for DummyPlaylistRepository {
    async fn get(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>> {
        Ok(Some(Playlist::new(
            id.clone(),
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            ProviderId::new(String::from("deezer")),
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        )))
    }

    async fn get_all(&self) -> PlaylistRepositoryResult<Vec<Playlist>> {
        Ok(vec![Playlist::new(
            PlaylistId::LikedSongs,
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            ProviderId::new(String::from("deezer")),
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        )])
    }

    async fn create(&self, name: &String) -> PlaylistRepositoryResult<Playlist> {
        Ok(Playlist::new(
            PlaylistId::Owned(String::from("deezer_playlist_id")),
            name.clone(),
            HashSet::new(),
            String::from("me"),
            5,
            ProviderId::new(String::from("deezer")),
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        ))
    }

    async fn delete(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Playlist> {
        Ok(Playlist::new(
            id.clone(),
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            ProviderId::new(String::from("deezer")),
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        ))
    }

    async fn add_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &Vec<String>,
    ) -> PlaylistRepositoryResult<()> {
        Ok(())
    }

    async fn delete_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &Vec<String>,
    ) -> PlaylistRepositoryResult<()> {
        Ok(())
    }
}
