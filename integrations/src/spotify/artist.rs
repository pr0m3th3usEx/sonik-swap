use serde::Deserialize;
use url::Url;

use super::common::{SpotifyExternalUrls, SpotifyFollowers, SpotifyImage};

#[derive(Debug, Deserialize)]
pub struct SpotifySimplifiedArtist {
    /// The Spotify ID for the artist.
    pub id: String,
    /// Known external URLs for this artist.
    pub external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint providing full details of the artist.
    pub href: Url,
    /// The name of the artist.
    pub name: String,
    /// The object type => "artist"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for the artist.
    pub uri: String,
}
#[derive(Debug, Deserialize)]
pub struct SpotifyArtist {
    /// The Spotify ID for the artist.
    pub id: String,
    /// Known external URLs for this artist.
    pub external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint providing full details of the artist.
    pub href: Url,
    /// The name of the artist.
    pub name: String,
    /// The object type => "artist"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for the artist.
    pub uri: String,
    /// Information about the followers of the artist.
    pub followers: SpotifyFollowers,
    /// A list of the genres the artist is associated with. If not yet classified, the array is empty.
    pub genres: Vec<String>,
    /// A link to the Web API endpoint providing full details of the artist.
    pub images: Vec<SpotifyImage>,
    /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular.
    /// The artist's popularity is calculated from the popularity of all the artist's tracks.
    pub popularity: u32,
}

#[cfg(test)]
mod tests {
    use crate::spotify::artist::SpotifyArtist;

    #[test]
    fn test_deserialize_artist() {
        let payload = include_str!("../../tests/spotify/payload_artist.json");
        let json = serde_json::from_str::<SpotifyArtist>(&payload).expect("valid json");

        assert_eq!(json.name, "Pitbull");
        assert_eq!(json.popularity, 83);
        assert_eq!(json._type, "artist");
    }
}
