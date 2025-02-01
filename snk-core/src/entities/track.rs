use std::collections::{HashMap, HashSet};

use url::Url;

use crate::value_objects::{product_id::ProductId, provider::provider_id::ProviderId};

use super::{album::Album, artist::Artist};

pub struct Track {
    ids: HashSet<ProductId>, // Track ids from external databases & providers (ISRC, UPC, EAN, Provider IDs...)
    name: String,            // Name of the track
    duration_ms: u32,        // Duration of the track
    provider_urls: HashMap<ProviderId, Url>, // External Provider Music URLs
}

impl Track {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        duration_ms: u32,
        provider_urls: HashMap<ProviderId, Url>,
    ) -> Self {
        Self {
            ids,
            name,
            duration_ms,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashSet<ProductId> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn duration_ms(&self) -> u32 {
        self.duration_ms
    }

    pub fn provider_urls(&self) -> &HashMap<ProviderId, Url> {
        &self.provider_urls
    }
}

pub struct TrackWithAlbumAndArtists {
    pub ids: HashSet<ProductId>, // Track ids from external databases & providers (ISRC, UPC, EAP, Provider IDs...)
    pub name: String,            // Name of the track
    pub duration_ms: u32,        // Duration of the track
    pub provider_urls: HashMap<ProviderId, Url>, // External Provider Music URLs

    pub album: Album,         // Album which the track is part of
    pub artists: Vec<Artist>, // Artists present in the music
}

impl TrackWithAlbumAndArtists {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        duration_ms: u32,
        provider_urls: HashMap<ProviderId, Url>,
        album: Album,
        artists: Vec<Artist>,
    ) -> Self {
        Self {
            ids,
            name,
            duration_ms,
            provider_urls,
            album,
            artists,
        }
    }

    pub fn ids(&self) -> &HashSet<ProductId> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn duration_ms(&self) -> u32 {
        self.duration_ms
    }

    pub fn provider_urls(&self) -> &HashMap<ProviderId, Url> {
        &self.provider_urls
    }

    pub fn album(&self) -> &Album {
        &self.album
    }

    pub fn artists(&self) -> &Vec<Artist> {
        &self.artists
    }
}
