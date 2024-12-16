use thiserror::Error;
use uuid::Uuid;

use crate::entities::music_account_provider::MusicAccountProvider;

#[derive(Debug, Error)]
pub enum MusicAccountProviderRepositoryError {
    #[error("Service: {0}")]
    ServiceError(String),
}

pub type MusicAccountProviderRepositoryResult<T> = Result<T, MusicAccountProviderRepositoryError>;

/// Repository managing storage of Music Account Providers (streaming platforms)
/// available to use on SonikSwap
pub trait MusicAccountProviderRepository {
    /// Get provider information
    ///
    /// Arguments:
    /// - id: [`Uuid`]
    ///
    /// Returns:
    /// - [`Option<MusicAccountProvider>`] or [`MusicAccountProviderRepositoryError`]
    async fn get(
        &self,
        id: Uuid,
    ) -> MusicAccountProviderRepositoryResult<Option<MusicAccountProvider>>;

    /// Get list of all providers
    ///
    /// Returns:
    /// List of [`MusicAccountProvider`] or [`MusicAccountProviderRepositoryError`]
    async fn get_all(&self) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>>;

    /// Add provider
    ///
    /// Arguments:
    /// - provider: [`MusicAccountProvider`]
    ///
    /// Returns:
    /// if successful [`MusicAccountProvider`] otherwise [`MusicAccountProviderRepositoryError`]
    async fn add(
        &self,
        provider: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider>;

    /// Update provider information
    ///
    /// Arguments:
    /// - new: [`MusicAccountProvider`]
    ///
    /// Returns:
    /// if successful [`MusicAccountProvider`] otherwise [`MusicAccountProviderRepositoryError`]
    async fn update(
        &self,
        new: MusicAccountProvider,
    ) -> MusicAccountProviderRepositoryResult<MusicAccountProvider>;
}
