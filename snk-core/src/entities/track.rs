use std::collections::HashSet;

use crate::value_objects::{product_id::ProductId, provider_url::ProviderUrl};

use super::{album::Album, artist::Artist};

pub struct Track {
    ids: HashSet<ProductId>, // Track ids from external databases & providers (ISRC, UPC, EAN, Provider IDs...)
    name: String,            // Name of the track
    duration_ms: u32,        // Duration of the track
    provider_urls: HashSet<ProviderUrl>, // External Provider Music URLs
}

impl Track {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        duration_ms: u32,
        provider_urls: HashSet<ProviderUrl>,
    ) -> Self {
        Self {
            ids,
            name,
            duration_ms,
            provider_urls,
        }
    }
}

pub struct TrackWithAlbumAndArtists {
    ids: HashSet<ProductId>, // Track ids from external databases & providers (ISRC, UPC, EAP, Provider IDs...)
    name: String,            // Name of the track
    duration_ms: u32,        // Duration of the track
    provider_urls: HashSet<ProviderUrl>, // External Provider Music URLs

    album: Album,             // Album which the track is part of
    artists: HashSet<Artist>, // Artists present in the music
}

impl TrackWithAlbumAndArtists {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        duration_ms: u32,
        provider_urls: HashSet<ProviderUrl>,
        album: Album,
        artists: HashSet<Artist>,
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
}
