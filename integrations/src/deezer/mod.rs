pub mod album;
pub mod artist;
pub mod error;
pub mod playlist;
pub mod track;

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    time::Duration,
};

use error::{DeezerErrorPayload, DeezerErrorType};
use playlist::DeezerPlaylist;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use serde::Deserialize;
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
use track::DeezerTrack;
use url::Url;

static API_URL: &'static str = "https://api.deezer.com";

#[derive(Debug, Deserialize)]
pub struct DeezerList<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DeezerResponse {
    Error(DeezerErrorPayload),
    Playlist(DeezerPlaylist),
    ListPlaylists(DeezerList<DeezerPlaylist>),
    ListTracks(DeezerList<DeezerTrack>),
}

pub struct DeezerPlaylistRepository<'a> {
    http_client: Client,
    #[allow(dead_code)]
    music_account_provider: &'a MusicAccountProvider,
}

impl<'a> DeezerPlaylistRepository<'a> {
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
                .connect_timeout(Duration::from_secs(5)) // TODO Define const
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

impl<'a> PlaylistRepository for DeezerPlaylistRepository<'a> {
    async fn get(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>> {
        match id {
            PlaylistId::LikedSongs => Ok(Some(Playlist::new(
                id.clone(),
                "Liked songs".to_string(),
                HashSet::from_iter([ImageCover::Other(
                    Url::from_str(
                        "https://cdn.icon-icons.com/icons2/72/PNG/256/favourite_14390.png",
                    )
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?,
                )]),
                "me".to_string(),
                0, // TODO Get this information
                Url::from_str("https://www.deezer.com/us/profile/me/loved")
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?, // TODO Add user id
            ))),
            PlaylistId::Owned(deezer_id) => {
                let response = self
                    .http_client
                    .get(format!("{}/playlist/{}", API_URL, deezer_id))
                    .send()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

                match response.status() {
                    StatusCode::OK => {
                        let response_body =
                            response.json::<DeezerResponse>().await.map_err(|err| {
                                PlaylistRepositoryError::ServiceError(err.to_string())
                            })?;
                        match response_body {
                            DeezerResponse::Error(deezer_error) => {
                                let deezer_error = DeezerErrorType::try_from(deezer_error.error)
                                    .map_err(|err: &str| {
                                        PlaylistRepositoryError::ServiceError(err.to_string())
                                    })?;

                                match deezer_error {
                                    DeezerErrorType::DataNotFound => Ok(None),
                                    other_error => Err(PlaylistRepositoryError::ServiceError(
                                        other_error.to_string(),
                                    )),
                                }
                            }
                            DeezerResponse::Playlist(deezer_playlist) => {
                                Ok(Some(deezer_playlist.into()))
                            }
                            _ => Err(PlaylistRepositoryError::ServiceError(
                                "bad response format".to_string(),
                            )),
                        }
                    }
                    other => Err(PlaylistRepositoryError::ServiceError(format!(
                        "Failed request: {}",
                        other.to_string()
                    ))),
                }
            }
        }
    }

    async fn get_all(&self) -> PlaylistRepositoryResult<Vec<Playlist>> {
        let response = self
            .http_client
            .get(format!("{}/user/me/playlists", API_URL))
            .send()
            .await
            .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response
                    .json::<DeezerResponse>()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

                match response_body {
                    DeezerResponse::ListPlaylists(deezer_list_playlists) => {
                        Ok(deezer_list_playlists
                            .data
                            .into_iter()
                            .map(|p| Into::<Playlist>::into(p))
                            .collect())
                    }
                    DeezerResponse::Error(deezer_error_payload) => Err(
                        PlaylistRepositoryError::ServiceError(deezer_error_payload.error.message),
                    ),
                    _ => Err(PlaylistRepositoryError::ServiceError(
                        "bad response format".to_string(),
                    )),
                }
            }
            other => Err(PlaylistRepositoryError::ServiceError(format!(
                "Failed request: {}",
                other.to_string()
            ))),
        }
    }

    async fn create(&self, name: &String) -> PlaylistRepositoryResult<Playlist> {
        let mut payload = HashMap::new();

        payload.insert("title", name.as_str());
        let response = self
            .http_client
            .post(format!("{}/user/me/playlists", API_URL))
            .json(&payload)
            .send()
            .await
            .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response
                    .json::<DeezerResponse>()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

                match response_body {
                    DeezerResponse::Error(deezer_error_payload) => Err(
                        PlaylistRepositoryError::ServiceError(deezer_error_payload.error.message),
                    ),
                    DeezerResponse::Playlist(deezer_playlist) => Ok(deezer_playlist.into()),
                    _ => Err(PlaylistRepositoryError::ServiceError(
                        "bad response format".to_string(),
                    )),
                }
            }
            other => Err(PlaylistRepositoryError::ServiceError(format!(
                "Failed request: {}",
                other.to_string()
            ))),
        }
    }

    async fn delete(&self, playlist_id: &PlaylistId) -> PlaylistRepositoryResult<Playlist> {
        match playlist_id {
            PlaylistId::LikedSongs => Err(PlaylistRepositoryError::ServiceError(
                "operation not permitted with favourite tracks list".to_string(),
            )),
            PlaylistId::Owned(deezer_id) => {
                let response = self
                    .http_client
                    .delete(format!("{}/playlist/{}", API_URL, deezer_id))
                    .send()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;
                match response.status() {
                    StatusCode::OK => {
                        let response_body =
                            response.json::<DeezerResponse>().await.map_err(|err| {
                                PlaylistRepositoryError::ServiceError(err.to_string())
                            })?;

                        match response_body {
                            DeezerResponse::Error(deezer_error_payload) => {
                                Err(PlaylistRepositoryError::ServiceError(
                                    deezer_error_payload.error.message,
                                ))
                            }
                            DeezerResponse::Playlist(deezer_playlist) => Ok(deezer_playlist.into()),
                            _ => Err(PlaylistRepositoryError::ServiceError(
                                "bad response format".to_string(),
                            )),
                        }
                    }
                    other => Err(PlaylistRepositoryError::ServiceError(format!(
                        "Failed request: {}",
                        other.to_string()
                    ))),
                }
            }
        }
    }

    async fn add_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &Vec<String>,
    ) -> PlaylistRepositoryResult<()> {
        let mut data = HashMap::new();

        data.insert("songs", ids.join(","));

        match playlist_id {
            PlaylistId::LikedSongs => {
                todo!()
            }
            PlaylistId::Owned(deezer_id) => {
                let response = self
                    .http_client
                    .post(format!("{}/playlist/{}/tracks", API_URL, deezer_id))
                    .json(&data)
                    .send()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

                match response.status() {
                    StatusCode::OK => {
                        let response_body =
                            response.json::<DeezerResponse>().await.map_err(|err| {
                                PlaylistRepositoryError::ServiceError(err.to_string())
                            })?;

                        match response_body {
                            DeezerResponse::Error(deezer_error_payload) => {
                                Err(PlaylistRepositoryError::ServiceError(
                                    deezer_error_payload.error.message,
                                ))
                            }
                            _ => Ok(()),
                        }
                    }
                    other => Err(PlaylistRepositoryError::ServiceError(format!(
                        "Failed request: {}",
                        other.to_string()
                    ))),
                }
            }
        }
    }

    async fn delete_tracks(
        &self,
        playlist_id: &PlaylistId,
        ids: &Vec<String>,
    ) -> PlaylistRepositoryResult<()> {
        let mut data = HashMap::new();

        data.insert("songs", ids.join(","));

        match playlist_id {
            PlaylistId::LikedSongs => {
                todo!()
            }
            PlaylistId::Owned(_) => {
                todo!()
            }
        }
    }

    async fn get_tracks(
        &self,
        playlist_id: &PlaylistId,
    ) -> PlaylistRepositoryResult<Vec<TrackWithAlbumAndArtists>> {
        let endpoint = match playlist_id {
            PlaylistId::LikedSongs => format!("{}/user/me/tracks", API_URL),
            PlaylistId::Owned(deezer_id) => format!("{}/playlist/{}/tracks", API_URL, deezer_id),
        };

        let response = self
            .http_client
            .get(endpoint)
            .send()
            .await
            .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response
                    .json::<DeezerResponse>()
                    .await
                    .map_err(|err| PlaylistRepositoryError::ServiceError(err.to_string()))?;

                match response_body {
                    DeezerResponse::Error(deezer_error_payload) => Err(
                        PlaylistRepositoryError::ServiceError(deezer_error_payload.error.message),
                    ),
                    DeezerResponse::ListTracks(deezer_list_tracks) => {
                        let tracks = deezer_list_tracks
                            .data
                            .into_iter()
                            .map(|track| track.try_into())
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|err: &'static str| {
                                PlaylistRepositoryError::ServiceError(err.to_string())
                            })?;

                        Ok(tracks)
                    }
                    _ => Err(PlaylistRepositoryError::ServiceError(
                        "bad response format".to_string(),
                    )),
                }
            }
            other => Err(PlaylistRepositoryError::ServiceError(format!(
                "Failed request: {}",
                other.to_string()
            ))),
        }
    }
}
