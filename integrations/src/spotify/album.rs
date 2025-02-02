use std::str::FromStr;

use serde::Deserialize;
use url::Url;

use super::{
    artist::SpotifySimplifiedArtist,
    common::{
        SpotifyCopyright, SpotifyExternalIds, SpotifyExternalUrls, SpotifyImage,
        SpotifyReleaseDatePrecision, SpotifyRestriction,
    },
    track::SpotifySimplifiedTrack,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyAlbumType {
    Album,
    Single,
    Compilation,
}

impl FromStr for SpotifyAlbumType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(Self::Album),
            "single" => Ok(Self::Single),
            "compilation" => Ok(Self::Compilation),
            _ => Err("SpotifyAlbumType: invalid value"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotifyAlbumTracks {
    /// A link to the Web API endpoint returning the full result of the request
    pub href: Url,
    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: u32,
    /// URL to the next page of items. ( null if none)
    pub next: Option<Url>,
    /// URL to the previous page of items. ( null if none)
    pub previous: Option<Url>,
    /// The offset of the items returned (as set in the query or by default)
    pub offset: u32,
    /// The total number of items available to return.
    pub total: u32,
    /// List of tracks (simplified)
    pub items: Vec<SpotifySimplifiedTrack>,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyAlbum {
    /// The Spotify ID for the album.
    pub id: String,
    /// The type of the album.
    pub album_type: SpotifyAlbumType,
    /// The number of tracks in the album.
    pub total_tracks: u32,
    /// The markets in which the album is available: ISO 3166-1 alpha-2 country codes.
    /// NOTE: an album is considered available in a market when at least 1 of its tracks is available in that market.
    pub available_markets: Vec<String>,
    /// Known external URLs for this album.
    pub external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint providing full details of the album.
    pub href: Url,
    /// The cover art for the album in various sizes, widest first.
    pub images: Vec<SpotifyImage>,
    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub name: String,
    /// The date the album was first released.
    pub release_date: String,
    /// The precision with which release_date value is known.
    pub release_date_precision: SpotifyReleaseDatePrecision,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<SpotifyRestriction>,
    /// The object type => "album"
    #[serde(alias = "type")]
    pub _type: String,
    /// The artists of the album.
    /// Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SpotifySimplifiedArtist>,
    /// The tracks of the album.
    pub tracks: SpotifyAlbumTracks,
    /// The copyright statements of the album.
    pub copyrights: Vec<SpotifyCopyright>,
    /// Known external IDs for the album.
    pub external_ids: SpotifyExternalIds,
    /// The label associated with the album.
    pub label: String,
    /// The popularity of the album. The value will be between 0 and 100, with 100 being the most popular.
    pub popularity: u32,
}

#[cfg(test)]
pub mod tests {
    use crate::spotify::album::SpotifyAlbum;

    #[test]
    fn test_deserialize_album() {
        let payload = include_str!("../../tests/spotify/payload_album.json");
        let json = serde_json::from_str::<SpotifyAlbum>(&payload).expect("valid json");

        assert_eq!(json.name, "Global Warming");
        assert_eq!(json.external_ids.upc, Some("886443671584".to_string()));
    }
}
