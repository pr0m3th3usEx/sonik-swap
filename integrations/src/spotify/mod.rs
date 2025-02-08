use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use common::SpotifyList;
use playlist::{SpotifyPlaylist, SpotifySimplifiedPlaylist};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;
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
    // TODO Might chant that to metadata of current connected account
    /// Username of the Spotify account connected
    username: String,
}

impl<'a> SpotifyPlaylistRepository<'a> {
    pub fn new(
        music_account_provider: &'a MusicAccountProvider,
        username: String,
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
            username,
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
            Ok(res) => match id {
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
                                .map_err(|err| {
                                    PlaylistRepositoryError::ServiceError(err.to_string())
                                })?,
                        )]),
                        "me".to_string(),
                        favorite_tracks.total,
                        "https://open.spotify.com/collection/tracks"
                            .parse::<Url>()
                            .map_err(|err| {
                                PlaylistRepositoryError::ServiceError(err.to_string())
                            })?,
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
            },
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
        let url = format!("{}/me/playlists", API_URL);

        let response = self.http_client.get(url).send().await.map_err(|err| {
            PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Failed to fetch request - {:?}",
                err
            ))
        })?;

        match response.error_for_status() {
            Ok(res) => {
                let playlists = res
                    .json::<SpotifyList<SpotifySimplifiedPlaylist>>()
                    .await
                    .map_err(|err| {
                        PlaylistRepositoryError::ServiceError(format!(
                            "PlaylistRepository - Failed to parse response - {:?}",
                            err
                        ))
                    })?;

                Ok(playlists
                    .items
                    .into_iter()
                    .map(|playlist| playlist.into())
                    .collect())
            }
            Err(err) => Err(PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Error during request - {:?}",
                err
            ))),
        }
    }

    async fn create(&self, name: &str) -> PlaylistRepositoryResult<Playlist> {
        let url = format!("{}/user/{}/playlists", API_URL, self.username);

        let mut body: HashMap<&str, &str> = HashMap::new();

        body.insert("name", name);
        body.insert("description", "Playlist created thanks SonikSwap");

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|err| {
                PlaylistRepositoryError::ServiceError(format!(
                    "PlaylistRepository - Failed to send request - {:?}",
                    err
                ))
            })?;

        match response.error_for_status() {
            Ok(res) => {
                let playlist = res.json::<SpotifyPlaylist>().await.map_err(|err| {
                    PlaylistRepositoryError::ServiceError(format!(
                        "PlaylistRepository - Failed to parse response - {:?}",
                        err
                    ))
                })?;

                Ok(playlist.into())
            }
            Err(err) => Err(PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Error during request - {:?}",
                err
            ))),
        }
    }

    async fn delete(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>> {
        match id {
            PlaylistId::LikedSongs => Err(PlaylistRepositoryError::ServiceError(
                "operation not permitted with favourite tracks list".to_string(),
            )),
            PlaylistId::Owned(spotify_id) => {
                let url = format!("{}/playlists/{}/followers", API_URL, spotify_id);

                let response = self.http_client.delete(url).send().await.map_err(|err| {
                    PlaylistRepositoryError::ServiceError(format!(
                        "PlaylistRepository - Failed to send request - {:?}",
                        err
                    ))
                })?;

                match response.error_for_status() {
                    Ok(_) => Ok(None),
                    Err(err) => Err(PlaylistRepositoryError::ServiceError(format!(
                        "PlaylistRepository - Error during request - {:?}",
                        err
                    ))),
                }
            }
        }
    }

    async fn add_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &[String],
        snapshot_id: Option<String>,
    ) -> PlaylistRepositoryResult<()> {
        let response = match playlist_id {
            PlaylistId::LikedSongs => {
                let mut payload = HashMap::new();
                let url = format!("{}/me/tracks", API_URL);

                payload.insert(
                    "ids",
                    ids.iter()
                        .map(|id| format!("spotify:track:{}", id))
                        .collect::<Vec<String>>(),
                );

                self.http_client
                    .post(url)
                    .json(&payload)
                    .send()
                    .await
                    .map_err(|err| {
                        PlaylistRepositoryError::ServiceError(format!(
                            "PlaylistRepository - Failed to send request - {:?}",
                            err
                        ))
                    })?
            }
            PlaylistId::Owned(spotify_id) => {
                let url = format!("{}/playlists/{}/tracks", API_URL, spotify_id);
                let uris = ids
                    .iter()
                    .map(|id| format!("spotify:track:{}", id))
                    .collect::<Vec<String>>();

                self.http_client
                    .post(url)
                    .json(&json!({
                        "ids": uris,
                        "snapshot_id": snapshot_id,
                    }))
                    .send()
                    .await
                    .map_err(|err| {
                        PlaylistRepositoryError::ServiceError(format!(
                            "PlaylistRepository - Failed to send request - {:?}",
                            err
                        ))
                    })?
            }
        };

        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => Err(PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Error during request - {:?}",
                err
            ))),
        }
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
        let url = match playlist_id {
            PlaylistId::LikedSongs => format!("{}/me/tracks", API_URL),
            PlaylistId::Owned(spotify_id) => format!("{}/playlist/{}", API_URL, spotify_id),
        };

        let response = self.http_client.get(url).send().await.map_err(|err| {
            PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Failed to fetch request - {:?}",
                err
            ))
        })?;

        match response.error_for_status() {
            Ok(res) => match playlist_id {
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

                    // TODO Fetch all tracks
                    Ok(favorite_tracks
                        .items
                        .into_iter()
                        .map(|track| TrackWithAlbumAndArtists::from(track.track))
                        .collect())
                }
                PlaylistId::Owned(_) => {
                    let playlist = res.json::<SpotifyPlaylist>().await.map_err(|err| {
                        PlaylistRepositoryError::ServiceError(format!(
                            "PlaylistRepository - Failed to parse response - {:?}",
                            err
                        ))
                    })?;

                    // TODO Fetch all tracks
                    Ok(playlist
                        .tracks
                        .items
                        .into_iter()
                        .map(|track| TrackWithAlbumAndArtists::from(track.track))
                        .collect())
                }
            },
            Err(err) => Err(PlaylistRepositoryError::ServiceError(format!(
                "PlaylistRepository - Error during request - {:?}",
                err
            ))),
        }
    }
}
