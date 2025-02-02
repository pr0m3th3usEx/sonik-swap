use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use super::{
    album::{SpotifyAlbum, SpotifyPlaylistTrackAlbum}, artist::SpotifySimplifiedArtist, common::{SpotifyExternalIds, SpotifyExternalUrls, SpotifyFollowers, SpotifyRestriction}
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

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistTrackAddedBy {
    /// Known public external URLs for this user.
    pub external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint for this user.
    pub href: Option<Url>,
    /// The Spotify user ID for this user.
    pub id: String,
    /// Allowed values: "user"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for this user.
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistTrack {
    /// The date and time the track or episode was added.
    /// Note: some very old playlists may return null in this field.
    pub added_at: DateTime<Utc>,
    /// Whether this track or episode is a local file or not.
    pub added_by: SpotifyPlaylistTrackAddedBy,
    /// Whether this track or episode is a local file or not.
    pub is_local: bool,
    /// Information about the track or episode. 
    /// We only handling when it's a track
    pub track: SpotifyPlaylistTrackData,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistTrackData {
    /// The album on which the track appears. The album object includes a link in href to full information about the album.
    pub album: SpotifyPlaylistTrackAlbum,
    /// The artists who performed the track.
    /// Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SpotifySimplifiedArtist>,
    /// A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub available_markets: Vec<String>,
    /// The disc number (usually 1 unless the album consists of more than one disc).
    pub disc_number: u32,
    /// The track length in milliseconds
    pub duration_ms: u32,
    /// Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// Known external IDs for the track
    pub external_ids: SpotifyExternalIds,
    /// Known external URLs for this track.
    pub external_urls: SpotifyExternalUrls,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: Url,
    /// The Spotify ID of the track
    pub id: String,
    /// Part of the response when Track Relinking is applied.
    /// If true, the track is playable in the given market. Otherwise false.
    pub is_playable: Option<bool>,
    /// Part of the response when Track Relinking is applied, and the requested track has been replaced with different track. The track in the linked_from object contains information about the originally requested track.
    pub linked_from: Option<SpotifyLinkedTrack>,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<SpotifyRestriction>,
    /// The name of the track.
    pub name: String,
    /// The popularity of the track. 
    pub popularity: u32,
    /// A link to a 30 second preview (MP3 format) of the track. Can be null
    #[deprecated]
    pub preview_url: Option<Url>,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: u32,
    /// The object type => "track"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI of the track
    pub uri: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize_track() {
        todo!()
    }
}
