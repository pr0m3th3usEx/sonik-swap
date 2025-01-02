use std::collections::HashSet;

use serde::Deserialize;
use snk_core::{
    entities::playlist::Playlist,
    value_objects::{image_cover::ImageCover, playlist_id::PlaylistId},
};
use url::Url;

use super::artist::ReducedArtist;

#[derive(Debug, Deserialize)]
pub struct DeezerPlaylist {
    // The playlist's Deezer id
    pub id: String,
    // The playlist's title
    pub title: String,
    // The playlist description
    #[allow(dead_code)]
    pub description: String,
    // The playlist's duration (seconds)
    #[allow(dead_code)]
    pub duration: u32,
    // If the playlist is public or not
    #[allow(dead_code)]
    pub public: bool,
    // If the playlist is the love tracks playlist
    #[allow(dead_code)]
    pub is_loved_track: bool,
    // If the playlist is collaborative or not
    #[allow(dead_code)]
    pub collaborative: bool,
    // Nb tracks in the playlist
    pub nb_tracks: u32,
    // Nb tracks not seen
    #[allow(dead_code)]
    pub unseen_track_count: u32,
    // The number of playlist's fans
    #[allow(dead_code)]
    pub fans: u32,
    // The url of the playlist on Deezer
    pub link: Url,
    // The share link of the playlist on Deezer
    #[allow(dead_code)]
    pub share: Url,
    // The url of the playlist's cover. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    pub picture_small: Option<Url>,
    pub picture_medium: Option<Url>,
    pub picture_big: Option<Url>,
    pub picture_xl: Option<Url>,
    // The checksum for the track list
    #[allow(dead_code)]
    pub checksum: String,
    pub creator: ReducedArtist,
}

impl Into<Playlist> for DeezerPlaylist {
    fn into(self) -> Playlist {
        let id = PlaylistId::Owned(self.id);
        let name = self.title;
        let owner = self.creator.name.expect("missing creator name");
        let total_songs = self.nb_tracks;
        let provider_url = self.link;

        let mut covers = HashSet::new();

        covers.insert(ImageCover::Default(self.picture));

        if let Some(picture_small) = self.picture_small {
            covers.insert(ImageCover::Sm(picture_small));
        }
        if let Some(picture_medium) = self.picture_medium {
            covers.insert(ImageCover::Md(picture_medium));
        }
        if let Some(picture_big) = self.picture_big {
            covers.insert(ImageCover::Lg(picture_big));
        }
        if let Some(picture_xl) = self.picture_xl {
            covers.insert(ImageCover::Lg(picture_xl));
        }

        Playlist::new(id, name, covers, owner, total_songs, provider_url)
    }
}
