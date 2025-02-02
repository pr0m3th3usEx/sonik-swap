use serde::Deserialize;
use url::Url;

use super::{common::{SpotifyExternalUrls, SpotifyFollowers, SpotifyImage}, track::SpotifyPlaylistTrack};

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistOwner {
  /// Known public external URLs for this user.
  pub external_urls: SpotifyExternalUrls,
  /// Information about the followers of this user.
  pub followers: Option<SpotifyFollowers>,
  /// A link to the Web API endpoint for this user.
  pub href: Url,
  /// The Spotify user ID for this user.
  pub id: String,
  /// The object type => "user"
  #[serde(alias = "type")]
  pub _type: String,
  /// The Spotify URI for this user.
  pub uri: String,
  /// The name displayed on the user's profile. null if not available.
  pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistTracks {
  pub href: Url,
  pub limit: u32,
  pub next: Option<Url>,
  pub previous: Option<Url>,
  pub offset: u32,
  pub total: u32,
  pub items: Vec<SpotifyPlaylistTrack>,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylist {
  /// The Spotify ID of the playlist.
  pub id: String,
  /// Known external URLs for this playlist.
  pub external_urls: SpotifyExternalUrls,
  /// The playlist description. Only returned for modified, verified playlists, otherwise null.
  pub description: Option<String>,
  /// true if the owner allows other users to modify the playlist.
  pub collaborative: bool,
  /// Information about the followers of the playlist.
  pub followers: SpotifyFollowers,
  /// A link to the Web API endpoint providing full details of the playlist.
  pub href: Url,
  /// Images for the playlist. 
  pub images: Vec<SpotifyImage>,
  /// The name of the playlist.
  pub name: String,
  /// The user who owns the playlist
  pub owner: SpotifyPlaylistOwner,
  /// The playlist's public/private status (if it is added to the user's profile): true
  /// the playlist is public, false the playlist is private, null the playlist status is not relevant.
  pub public: bool,
  /// The version identifier for the current playlist.
  /// Can be supplied in other requests to target a specific playlist version
  pub snapshot_id: String,
  /// The tracks of the playlist.
  pub tracks: SpotifyPlaylistTracks,
  /// The object type => "playlist"
  #[serde(alias = "type")]
  pub _type: String,
  /// The Spotify URI for the playlist
  pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::SpotifyPlaylist;

  #[test]
  fn test_deserialize_playlist() {
    let payload = include_str!("../../tests/spotify/payload_playlist.json");
    let json = serde_json::from_str::<SpotifyPlaylist>(&payload).expect("valid json");

    assert_eq!(json.name, "My Dearest OST");
  }
}