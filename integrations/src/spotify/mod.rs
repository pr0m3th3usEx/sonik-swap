use std::{collections::HashSet, time::Duration};

use common::SpotifyList;
use playlist::SpotifyPlaylist;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use snk_core::{
    contracts::repositories::playlist_repository::{
        PlaylistRepository, PlaylistRepositoryError, PlaylistRepositoryResult,
    },
    entities::{
        music_account_provider::MusicAccountProvider, playlist::Playlist,
        track::TrackWithAlbumAndArtists,
    },
    value_objects::{image_cover::ImageCover, playlist_id::PlaylistId},
};
use track::SpotifySavedTrack;
use url::Url;

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
        let url = match id {
            PlaylistId::LikedSongs => format!("{}/me/tracks", API_URL),
            PlaylistId::Owned(playlist_id) => format!("{}/playlist/{}", API_URL, playlist_id),
        };

        let response = self.http_client.get(url).send().await.map_err(|err| {
            PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Failed to fetch request - {:?}",
                err
            ))
        })?;

        match response.error_for_status() {
            Ok(res) => {
                match id {
                    PlaylistId::LikedSongs => {
                        let favorite_tracks = res
                            .json::<SpotifyList<SpotifySavedTrack>>()
                            .await
                            .map_err(|err| {
                                PlaylistRepositoryError::ServiceError(format!(
                                    "PlaylistRepository - Failed to parse response - {:?}",
                                    err
                                ))
                            })?;

                        Ok(Some(Playlist::new(
                            id.clone(),
                            id.to_string(),
                            HashSet::from_iter([ImageCover::Other(
                                "https://cdn.icon-icons.com/icons2/72/PNG/256/favourite_14390.png"
                                    .parse::<Url>()
                                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?,
                            )]),
                            "me".to_string(),

                            favorite_tracks.total,
                            "https://open.spotify.com/collection/tracks"
                                .parse::<Url>()
                                .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?
                        )))
                    }
                    PlaylistId::Owned(_) => {
                        let playlist = res.json::<SpotifyPlaylist>().await.map_err(|err| {
                            PlaylistRepositoryError::ServiceError(format!(
                                "PlaylistRepository - Failed to parse response - {:?}",
                                err
                            ))
                        })?;

                        Ok(Some(playlist.into()))
                    }
                }
            }
            Err(err) => {
                let Some(status) = err.status() else {
                    return Err(PlaylistRepositoryError::ServiceError(format!(
                        "PlaylistRepository - Error during request - {:?}",
                        err
                    )));
                };

                if status == 404 {
                    return Ok(None);
                }

                return Err(PlaylistRepositoryError::ServiceError(format!(
                    "PlaylistRepository - Error during request - {:?}",
                    err
                )));
            }
        }
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
