use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use snk_core::value_objects::provider::provider_id::ProviderId;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct SpotifyList<T> {
    #[allow(dead_code)]
    pub href: Url,
    #[allow(dead_code)]
    pub limit: u32,
    #[allow(dead_code)]
    pub next: Option<Url>,
    #[allow(dead_code)]
    pub previous: Option<Url>,
    #[allow(dead_code)]
    pub offset: u32,

    pub total: u32,
    pub items: Vec<T>,
}

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

pub struct SpotifyDateTimeWrapper(pub DateTime<Utc>);

impl From<(SpotifyReleaseDatePrecision, String)> for SpotifyDateTimeWrapper {
    fn from((precision, value): (SpotifyReleaseDatePrecision, String)) -> Self {
        SpotifyDateTimeWrapper(
            NaiveDateTime::parse_from_str(
                &value,
                match precision {
                    SpotifyReleaseDatePrecision::Year => "%Y",
                    SpotifyReleaseDatePrecision::Month => "%Y-%m",
                    SpotifyReleaseDatePrecision::Day => "%Y-%m-%d",
                },
            )
            .unwrap()
            .and_utc(),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: Url,
}

impl From<SpotifyExternalUrls> for HashMap<ProviderId, Url> {
    fn from(external_urls: SpotifyExternalUrls) -> Self {
        let mut map = HashMap::new();

        map.insert(
            ProviderId::new("spotify".to_string()),
            external_urls.spotify,
        );
        map
    }
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
    #[allow(dead_code)]
    reason: SpotifyRestrictionReason,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyCopyright {
    #[allow(dead_code)]
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

#[derive(Debug, Serialize)]
pub struct SpotifyUri {
    pub uri: String,
}
