use partially::Partial;
use serde::Deserialize;
use url::Url;

use super::artist::ReducedArtist;

#[derive(Debug, Deserialize, Partial)]
#[partially(derive(Debug, Deserialize))]
#[partially(rename = "ReducedAlbum")]
pub struct DeezerAlbum {
    // The Deezer album id
    pub id: String,
    // The album title
    pub title: String,
    // The album UPC
    pub upc: String,
    // The url of the album on Deezer
    pub link: Url,
    // The share link of the album on Deezer
    #[partially(omit)]
    pub share: Url,
    // The url of the album's cover. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub cover: Url,
    // The url of the album's cover in size small.
    pub cover_small: Url,
    // The url of the album's cover in size medium.
    pub cover_medium: Url,
    // The url of the album's cover in size big.
    pub cover_big: Url,
    // The url of the album's cover in size xl.
    pub cover_xl: Url,
    #[partially(omit)]
    pub md5_image: String,
    // The album's first genre id (You should use the genre list instead). NB : -1 for not found
    #[partially(omit)]
    pub genre_id: i32,
    // List of genre object
    // TODO Genre object (not covered)
    #[partially(omit)]
    pub genres: serde_json::Value,
    // The album's label name
    #[partially(omit)]
    pub label: String,
    #[partially(omit)]
    pub nb_tracks: u32,
    // The album's duration (seconds)
    #[partially(omit)]
    pub duration: u32,
    // The number of album's Fans
    #[partially(omit)]
    pub fans: u32,
    // The album's release date
    pub release_date: String,
    // The record type of the album (EP / ALBUM / etc..)
    #[partially(omit)]
    pub record_type: String,
    #[partially(omit)]
    pub available: bool,
    // API Link to the tracklist of this album
    #[partially(omit)]
    pub tracklist: Url,
    // Whether the album contains explicit lyrics
    #[partially(omit)]
    pub explicit_lyrics: bool,
    // The explicit content lyrics values (0:Not Explicit; 1:Explicit; 2:Unknown; 3:Edited; 6:No Advice Available)
    #[partially(omit)]
    pub explicit_content_lyrics: u32,
    // The explicit cover value (0:Not Explicit; 1:Explicit; 2:Unknown; 3:Edited; 6:No Advice Available)
    #[partially(omit)]
    pub explicit_content_cover: u32,
    // Return a list of contributors on the album
    #[partially(omit)]
    pub contributors: Vec<ReducedArtist>,
    // Return fallback album with id and status
    #[partially(omit)]
    pub fallback: Option<serde_json::Value>,
    // artist object containing : id, name, picture, picture_small, picture_medium, picture_big, picture_xl
    #[partially(omit)]
    pub artist: ReducedArtist,
}

#[cfg(test)]
mod tests {
    use crate::integrations::deezer::artist::DeezerIdType;

    use super::DeezerAlbum;

    #[test]
    pub fn test_deserialize_album() {
        let json_str = include_str!("../../../tests/deezer/payload_album.json");
        let json = serde_json::from_str::<DeezerAlbum>(&json_str).expect("valid json");

        assert_eq!(json.title, "How Sweet");
        assert_eq!(json.upc, "196922889738");
        assert_eq!(
            json.artist.id,
            Some(DeezerIdType::IdString("178008437".to_string()))
        );
    }
}
