use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, Hash)]
pub enum Provider {
  Spotify,
  Deezer
}

#[derive(Debug, Error)]
pub enum ProviderError {
  #[error("Unknown provider")]
  Unknown
}

impl FromStr for Provider {
    type Err = ProviderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
          "spotify" => Ok(Provider::Spotify),
          "deezer" => Ok(Provider::Deezer),
          _ => Err(ProviderError::Unknown)
        }
    }
}