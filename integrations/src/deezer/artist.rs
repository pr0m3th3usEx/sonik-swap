use std::fmt::Display;

use partially::Partial;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DeezerIdType {
    IdNumber(i32),
    IdString(String),
}

impl Display for DeezerIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DeezerIdType::IdNumber(id) => id.to_string(),
            DeezerIdType::IdString(id) => id.clone(),
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize, Partial)]
#[partially(derive(Debug, Deserialize))]
#[partially(rename = "ReducedArtist")]
pub struct DeezerArtist {
    pub id: DeezerIdType,
    pub name: String,
    pub link: Url,
    pub share: Url,
    pub picture: Url,
    pub picture_small: Url,
    pub picture_medium: Url,
    pub picture_big: Url,
    pub picture_xl: Url,
    pub nb_album: u32,
    pub nb_fan: u32,
    pub radio: bool,
    pub tracklist: Option<Url>,
}

#[cfg(test)]
mod tests {
    use super::{DeezerArtist, DeezerIdType};

    #[test]
    pub fn test_deserialize_artist() {
        let json_str = include_str!("../../tests/payloads/test_artist.json");
        let json = serde_json::from_str::<DeezerArtist>(&json_str).expect("valid json");

        assert_eq!(json.name, "NewJeans");
        assert_eq!(json.id, DeezerIdType::IdString("178008437".to_string()));
    }
}
