use error::DeezerErrorPayload;
use playlist::DeezerPlaylist;
use serde::Deserialize;
use track::DeezerTrack;
use user::DeezerUser;

pub mod album;
pub mod artist;
pub mod error;
pub mod playlist;
pub mod repositories;
pub mod track;
pub mod user;

static API_URL: &str = "https://api.deezer.com";

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
    Playlist(Box<DeezerPlaylist>),
    User(Box<DeezerUser>),
    ListPlaylists(DeezerList<DeezerPlaylist>),
    ListTracks(DeezerList<DeezerTrack>),
    ActionResult(()),
}

#[cfg(test)]
mod tests {
    use crate::integrations::deezer::API_URL;
    use std::collections::HashMap;

    #[test]
    pub fn test_url_with_params() {
        let data: HashMap<&'static str, &'static str> = HashMap::from_iter([("songs", "1,2")]);
        let result = reqwest::Url::parse_with_params(
            format!("{}/playlist/{}/tracks", API_URL, "10000000").as_str(),
            data,
        );

        assert!(result.is_ok());

        let url = result.unwrap();
        assert_eq!(
            url.to_string(),
            format!("{}/playlist/{}/tracks?songs=1%2C2", API_URL, "10000000").as_str()
        )
    }
}
