use serde::Deserialize;
use url::Url;

use super::{
    artist::SpotifySimplifiedArtist,
    common::{SpotifyExternalUrls, SpotifyRestriction},
};

#[derive(Debug, Deserialize)]
pub struct SpotifyLinkedTrack {
    /// The Spotify ID for the track.
    pub id: String,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: Url,
    /// Known external URLs for this track.
    pub external_urls: SpotifyExternalUrls,
    /// The object type => "track"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for the track.
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifySimplifiedTrack {
    /// The Spotify ID for the track.
    pub id: String,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: Url,
    /// External URLs for this track.
    pub external_urls: SpotifyExternalUrls,
    /// Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// The track length in milliseconds.
    pub duration_ms: u32,
    /// The disc number (usually 1 unless the album consists of more than one disc).
    pub disc_number: u32,
    /// A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub available_markets: Vec<String>,
    /// The artists who performed the track. Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SpotifySimplifiedArtist>,
    /// Part of the response when Track Relinking is applied. If true, the track is playable in the given market. Otherwise false.
    pub is_playable: Option<bool>,
    /// Part of the response when Track Relinking is applied and is only part of the response if the track linking, in fact, exists. The requested track has been replaced with a different track. The track in the linked_from object contains information about the originally requested track.
    pub linked_from: Option<SpotifyLinkedTrack>,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<SpotifyRestriction>,
    /// The name of the track.
    pub name: String,
    /// A URL to a 30 second preview (MP3 format) of the track.
    pub preview_url: Option<Url>,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: u32,
    /// The object type => "track"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for the track.
    pub uri: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
}
