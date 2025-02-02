use serde::Deserialize;
use url::Url;

use super::common::SpotifyExternalUrls;

#[derive(Debug, Deserialize)]
pub struct SpotifySimplifiedArtist {
    /// The Spotify ID for the artist.
    id: String,
    /// Known external URLs for this artist.
    external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint providing full details of the artist.
    href: Url,
    /// The name of the artist.
    name: String,
    /// The object type => "artist"
    #[serde(alias = "type")]
    _type: String,
    /// The Spotify URI for the artist.
    uri: String,
}
