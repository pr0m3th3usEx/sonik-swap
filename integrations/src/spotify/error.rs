use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpotifyError {
  #[allow(dead_code)]
  error: SpotifyErrorData,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyErrorData {
  #[allow(dead_code)]
  status: u32,
  #[allow(dead_code)]
  message: String,
}