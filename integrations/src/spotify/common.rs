use std::str::FromStr;

use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct SpotifyImage {
    /// The source URL of the image.
    pub url: Url,
    /// The image width in pixels.
    pub width: Option<u32>,
    /// The image height in pixels.
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyReleaseDatePrecision {
    Year,
    Month,
    Day,
}

impl FromStr for SpotifyReleaseDatePrecision {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "year" => Ok(Self::Year),
            "month" => Ok(Self::Month),
            "day" => Ok(Self::Day),
            _ => Err("SpotifyReleaseDatePrecision: invalid value"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyRestrictionReason {
    Market,
    Product,
    Explicit,
}

impl FromStr for SpotifyRestrictionReason {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "market" => Ok(Self::Market),
            "product" => Ok(Self::Product),
            "explicit" => Ok(Self::Explicit),
            _ => Err("SpotifyRestrictionReason: invalid value"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotifyRestriction {
    reason: SpotifyRestrictionReason,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyCopyright {
    pub text: String,
    #[serde(alias = "type")]
    pub _type: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyExternalIds {
    pub isrc: Option<String>,
    pub upc: Option<String>,
    pub ean: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyFollowers {
    /// This will always be set to null, as the Web API does not support it at the moment
    pub href: Option<Url>,
    /// The total number of followers.
    pub total: u32,
}
