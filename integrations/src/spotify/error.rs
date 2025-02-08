use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpotifyError {
  error: SpotifyErrorData,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorData {
  status: u32,
  message: String,
}