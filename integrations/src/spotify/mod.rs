use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use snk_core::{
    contracts::repositories::playlist_repository::{PlaylistRepository, PlaylistRepositoryResult},
    entities::{
        music_account_provider::MusicAccountProvider, playlist::Playlist,
        track::TrackWithAlbumAndArtists,
    },
    value_objects::playlist_id::PlaylistId,
};

mod album;
mod artist;
mod common;
mod error;
mod playlist;
mod track;

static API_URL: &str = "https://api.spotify.com/v1";

pub struct SpotifyPlaylistRepository<'a> {
    http_client: Client,
    #[allow(dead_code)]
    music_account_provider: &'a MusicAccountProvider,
}

impl<'a> SpotifyPlaylistRepository<'a> {
    pub fn new(
        music_account_provider: &'a MusicAccountProvider,
        access_token: String,
    ) -> Result<Self, &'static str> {
        let mut default_headers = HeaderMap::new();

        default_headers.insert("Accept", HeaderValue::from_static("application/json"));
        default_headers.insert(
            "Authorization",
            format!("Bearer: {}", access_token).parse().map_err(|err| {
                eprintln!("{:?}", err);
                "DeezerPlaylistRepository::new: Could not parse header value"
            })?,
        );

        Ok(Self {
            http_client: Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .default_headers(default_headers)
                .build()
                .map_err(|err| {
                    eprintln!("{:?}", err);
                    "DeezerPlaylistRepository::new: Could not init HTTP client"
                })?,
            music_account_provider,
        })
    }
}

impl<'a> PlaylistRepository for SpotifyPlaylistRepository<'a> {
    async fn get(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>> {
        todo!()
    }

    async fn get_all(&self) -> PlaylistRepositoryResult<Vec<Playlist>> {
        todo!()
    }

    async fn create(&self, name: &str) -> PlaylistRepositoryResult<Playlist> {
        todo!()
    }

    async fn delete(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Playlist> {
        todo!()
    }

    async fn add_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &[String],
    ) -> PlaylistRepositoryResult<()> {
        todo!()
    }

    async fn delete_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &[String],
    ) -> PlaylistRepositoryResult<()> {
        todo!()
    }

    async fn get_tracks(
        &self,
        playlist_id: &PlaylistId,
    ) -> PlaylistRepositoryResult<Vec<TrackWithAlbumAndArtists>> {
        todo!()
    }
}
