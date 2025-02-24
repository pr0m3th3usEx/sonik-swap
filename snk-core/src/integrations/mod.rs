use deezer::repositories::account_repository::DeezerProviderAccountRepository;
use spotify::repositories::account_repository::SpotifyProviderAccountRepository;

use crate::{
    contracts::repositories::provider_account_repository::ProviderAccountRepository,
    value_objects::provider::provider_id::ProviderId,
};

pub mod deezer;
pub mod spotify;

pub fn get_account_provider_repo(
    provider_id: &ProviderId,
    access_token: String,
) -> Result<Box<dyn ProviderAccountRepository>, &'static str> {
    match provider_id.as_str() {
        "spotify" => SpotifyProviderAccountRepository::new(access_token)
            .map(|repo: SpotifyProviderAccountRepository| {
                Box::new(repo) as Box<dyn ProviderAccountRepository + 'static>
            })
            .map_err(|_| "error while creating repo"),
        "deezer" => DeezerProviderAccountRepository::new(access_token)
            .map(|repo| Box::new(repo) as Box<dyn ProviderAccountRepository + 'static>)
            .map_err(|_| "error while creating repo"),
        _ => Err("unknown provider"),
    }
}
