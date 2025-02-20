use thiserror::Error;

use crate::{entities::music_account_provider::MusicAccountProvider, value_objects::provider::provider_id::ProviderId};

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
    /// - id: [`&ProviderId`]
    ///
    /// Returns:
    /// - [`Option<MusicAccountProvider>`] or [`MusicAccountProviderRepositoryError`]
    async fn get(
        &self,
        id: &ProviderId,
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

    /// Get Providers used for authentication
    ///
    /// Arguments: None
    /// 
    /// Returns:
    /// if successful [`Vec<MusicAccountProvider>`] otherwise [`MusicAccountProviderRepositoryError`]
    async fn get_auth_providers(&self) -> MusicAccountProviderRepositoryResult<Vec<MusicAccountProvider>>;
}
