use thiserror::Error;

use crate::{entities::playlist::Playlist, value_objects::playlist_id::PlaylistId};

#[derive(Debug, Error)]
pub enum PlaylistRepositoryError {
  #[error("ServiceError: {0}")]
  ServiceError(String),
}

pub type PlaylistRepositoryResult<T> = Result<T, PlaylistRepositoryError>;

pub trait PlaylistRepository {
  async fn get(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>>;

  async fn get_all(&self) -> PlaylistRepositoryResult<Vec<Playlist>>;

  async fn create(&self, name: &String) -> PlaylistRepositoryResult<Playlist>;

  async fn delete(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Playlist>;

  async fn add_tracks(&self, playlist_id: &PlaylistId, ids: &Vec<String>) -> PlaylistRepositoryResult<()>;

  async fn delete_tracks(&self, playlist_id: &PlaylistId, ids: &Vec<String>) -> PlaylistRepositoryResult<()>;
}