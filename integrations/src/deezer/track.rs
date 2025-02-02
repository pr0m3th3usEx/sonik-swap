use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use serde::Deserialize;
use snk_core::{
    entities::{album::Album, artist::Artist, track::TrackWithAlbumAndArtists},
    value_objects::{
        image_cover::ImageCover, product_id::ProductId, provider::provider_id::ProviderId,
    },
};
use url::Url;

use super::{album::ReducedAlbum, artist::ReducedArtist};

#[derive(Debug, Deserialize)]
pub struct DeezerTrack {
    // The track's Deezer id
    pub id: String,
    // true if the track is readable in the player for the current user
    pub readable: bool,
    // The track's fulltitle
    pub title: String,
    // The track's short title
    pub title_short: String,
    // The track version
    pub title_version: String,
    // The track isrc
    pub isrc: String,
    // The url of the track on Deezer
    pub link: Url,
    // The share link of the track on Deezer
    pub share: Url,
    // The track's duration in seconds
    pub duration: String,
    // The position of the track in its album
    pub track_position: u32,
    // The track's album's disk number
    pub disk_number: u32,
    // The track's Deezer rank
    pub rank: String,
    // The track's release date
    pub release_date: String,
    // Whether the track contains explicit lyrics
    pub explicit_lyrics: bool,
    // The explicit content lyrics values (0:Not Explicit; 1:Explicit; 2:Unknown; 3:Edited; 6:No Advice Available)
    pub explicit_content_lyrics: u32,
    // The explicit cover value (0:Not Explicit; 1:Explicit; 2:Unknown; 3:Edited; 6:No Advice Available)
    pub explicit_content_cover: u32,
    // The url of track's preview file. This file contains the first 30 seconds of the track
    pub preview: Url,
    // Beats per minute
    pub bpm: f32,
    // Signal strength
    pub gain: f32,
    // List of countries where the track is available
    pub available_countries: Vec<String>,
    // Return a list of contributors on the track
    pub contributors: Vec<ReducedArtist>,
    pub md5_image: String,
    // The track token for media service
    pub track_token: String,
    // artist object containing : id, name, link, share, picture, picture_small, picture_medium, picture_big, picture_xl, nb_album, nb_fan, radio, tracklist, role
    pub artist: ReducedArtist,
    // album object containing : id, title, link, cover, cover_small, cover_medium, cover_big, cover_xl, release_date
    pub album: ReducedAlbum,
}

impl TryInto<TrackWithAlbumAndArtists> for DeezerTrack {
    type Error = &'static str;

    fn try_into(self) -> Result<TrackWithAlbumAndArtists, Self::Error> {
        let mut ids = HashSet::new();

        ids.insert(ProductId::ISRC(self.isrc));
        ids.insert(ProductId::Provider((
            ProviderId::new("deezer".to_string()),
            self.id,
        )));

        let Ok(duration) = self.duration.parse::<u32>() else {
            return Err("duration is corrupted");
        };

        let mut urls = HashMap::new();

        urls.insert(ProviderId::new("deezer".to_string()), self.link);

        let album = get_album(self.album)?;
        let artists = get_artists(self.contributors)?;

        Ok(TrackWithAlbumAndArtists::new(
            ids,
            self.title,
            duration * 1000,
            urls,
            album,
            artists,
        ))
    }
}

fn get_album(reduced_album: ReducedAlbum) -> Result<Album, &'static str> {
    let Some(album_id) = reduced_album.id else {
        return Err("album.id is missing");
    };

    let Some(album_upc) = reduced_album.upc else {
        return Err("album.upc is missing");
    };

    let mut album_ids = HashSet::new();

    album_ids.insert(ProductId::UPC(album_upc));
    album_ids.insert(ProductId::Provider((
        ProviderId::new("deezer".to_string()),
        album_id,
    )));

    let Some(album_title) = reduced_album.title else {
        return Err("album.title is missing");
    };

    // Album

    let Some(album_cover_default) = reduced_album.cover else {
        return Err("album.cover is missing");
    };

    let Some(album_cover_small) = reduced_album.cover_small else {
        return Err("album.cover_small is missing");
    };

    let Some(album_cover_medium) = reduced_album.cover_medium else {
        return Err("album.cover_medium is missing");
    };

    let Some(album_cover_big) = reduced_album.cover_big else {
        return Err("album.cover_big is missing");
    };

    let Some(album_cover_xl) = reduced_album.cover_xl else {
        return Err("album.cover_xl is missing");
    };

    let mut album_covers = HashSet::new();

    album_covers.insert(ImageCover::Default(album_cover_default));
    album_covers.insert(ImageCover::Sm(album_cover_small));
    album_covers.insert(ImageCover::Md(album_cover_medium));
    album_covers.insert(ImageCover::Lg(album_cover_big));
    album_covers.insert(ImageCover::Other(album_cover_xl));

    let Some(album_release_date) = reduced_album.release_date else {
        return Err("album.release_date is missing");
    };
    let Ok(album_release_date) = NaiveDateTime::parse_from_str(&album_release_date, "%Y-%m-%d")
    else {
        return Err("album.release_date is corrupted");
    };

    let Some(album_link) = reduced_album.link else {
        return Err("album.link is missing");
    };

    let mut album_urls = HashMap::new();

    album_urls.insert(ProviderId::new("deezer".to_string()), album_link);

    let album = Album::new(
        album_ids,
        album_title,
        album_release_date.and_utc(),
        album_covers,
        album_urls,
    );

    Ok(album)
}

fn get_artists(reduced_artists: Vec<ReducedArtist>) -> Result<Vec<Artist>, &'static str> {
    reduced_artists
        .into_iter()
        .map(|reduced| {
            let mut ids = HashMap::new();

            let Some(id) = reduced.id else {
                return Err("artist.id is missing");
            };

            ids.insert(ProviderId::new("deezer".to_string()), id.to_string());

            let Some(name) = reduced.name else {
                return Err("artist.name is missing");
            };

            let Some(picture_default) = reduced.picture else {
                return Err("artist.picture is missing");
            };

            let Some(picture_small) = reduced.picture_small else {
                return Err("artist.picture_small is missing");
            };

            let Some(picture_medium) = reduced.picture_medium else {
                return Err("artist.picture_medium is missing");
            };

            let Some(picture_big) = reduced.picture_big else {
                return Err("artist.picture_big is missing");
            };

            let Some(picture_xl) = reduced.picture_xl else {
                return Err("artist.picture_xl is missing");
            };

            let mut pictures = HashSet::new();

            pictures.insert(ImageCover::Default(picture_default));
            pictures.insert(ImageCover::Sm(picture_small));
            pictures.insert(ImageCover::Md(picture_medium));
            pictures.insert(ImageCover::Lg(picture_big));
            pictures.insert(ImageCover::Other(picture_xl));

            let mut urls = HashMap::new();

            let Some(link) = reduced.link else {
                return Err("artist.link is missing");
            };

            urls.insert(ProviderId::new("deezer".to_string()), link);

            Ok(Artist::new(ids, name, pictures, urls))
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use crate::deezer::track::DeezerTrack;

    #[test]
    pub fn test_deserialize_playlist() {
        let json_str = include_str!("../../tests/deezer/payload_track.json");
        let json = serde_json::from_str::<DeezerTrack>(&json_str).expect("valid json");

        assert_eq!(json.title, "How Sweet");
        assert_eq!(json.artist.name, Some("NewJeans".to_string()));
        assert_eq!(json.album.title, Some("How Sweet".to_string()));
    }
}
