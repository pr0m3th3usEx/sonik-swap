use std::collections::HashSet;

use serde::Deserialize;
use snk_core::{
    entities::playlist::Playlist,
    value_objects::{image_cover::ImageCover, playlist_id::PlaylistId},
};
use url::Url;

use super::{
    common::{SpotifyExternalUrls, SpotifyFollowers, SpotifyImage},
    track::SpotifyPlaylistTrack,
};

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
pub struct SpotifySimplifiedPlaylistTracks {
    pub href: Url,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpotifySimplifiedPlaylist {
    /// The Spotify ID of the playlist.
    pub id: String,
    /// true if the owner allows other users to modify the playlist.
    pub collaborative: bool,
    /// The playlist description. Only returned for modified, verified playlists, otherwise null.
    pub description: String,
    /// Known external URLs for this playlist.
    pub external_urls: SpotifyExternalUrls,
    /// Images for the playlist.
    pub href: Url,
    /// Images for the playlist.
    pub images: Vec<SpotifyImage>,
    /// The playlist's public/private status (if it is added to the user's profile): true
    /// the playlist is public, false the playlist is private, null the playlist status is not relevant.
    pub public: bool,
    /// The name of the playlist.
    pub name: String,
    /// The user who owns the playlist
    pub owner: SpotifyPlaylistOwner,
    /// The object type => "playlist"
    #[serde(alias = "type")]
    pub _type: String,
    /// The Spotify URI for the playlist
    pub uri: String,
    /// The tracks of the playlist.
    pub tracks: SpotifySimplifiedPlaylistTracks,
}

impl From<SpotifySimplifiedPlaylist> for Playlist {
    fn from(spotify_playlist: SpotifySimplifiedPlaylist) -> Self {
        let playlist_id = PlaylistId::Owned(spotify_playlist.id);
        let name = spotify_playlist.name;
        let owner = spotify_playlist.owner.display_name;
        let total_songs = spotify_playlist.tracks.total;
        let provider_url = spotify_playlist.external_urls.spotify;
        let mut covers: HashSet<ImageCover> = HashSet::new();

        // The array may be empty or contain up to three images. The images are returned by size in descending order
        let mut iter = spotify_playlist.images.into_iter();

        // Default & large cover
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Default(image.url.clone()));
            covers.insert(ImageCover::Lg(image.url));
        }

        // Medium
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Md(image.url));
        }

        // Small
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Sm(image.url));
        }

        Playlist::new(playlist_id, name, covers, owner, total_songs, provider_url)
    }
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

impl From<SpotifyPlaylist> for Playlist {
    fn from(spotify_playlist: SpotifyPlaylist) -> Self {
        let playlist_id = PlaylistId::Owned(spotify_playlist.id);
        let name = spotify_playlist.name;
        let owner = spotify_playlist.owner.display_name;
        let total_songs = spotify_playlist.tracks.total;
        let provider_url = spotify_playlist.external_urls.spotify;
        let mut covers: HashSet<ImageCover> = HashSet::new();

        // The array may be empty or contain up to three images. The images are returned by size in descending order
        let mut iter = spotify_playlist.images.into_iter();

        // Default & large cover
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Default(image.url.clone()));
            covers.insert(ImageCover::Lg(image.url));
        }

        // Medium
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Md(image.url));
        }

        // Small
        if let Some(image) = iter.next() {
            covers.insert(ImageCover::Sm(image.url));
        }

        Playlist::new(playlist_id, name, covers, owner, total_songs, provider_url)
    }
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
