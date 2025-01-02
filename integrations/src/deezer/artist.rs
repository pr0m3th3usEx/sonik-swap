use partially::Partial;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize, Partial)]
#[partially(derive(Debug, Deserialize))]
#[partially(rename = "ReducedArtist")]
pub struct DeezerArtist {
    pub id: String,
    pub name: String,
    pub link: Url,
    pub share: Url,
    pub picture: Url,
    pub picture_small: Url,
    pub picture_medium: Url,
    pub picture_big: Url,
    pub picture_xl: Url,
    pub nb_album: u32,
    pub nb_fan: u32,
    pub radio: bool,
    pub tracklist: Url,
}
