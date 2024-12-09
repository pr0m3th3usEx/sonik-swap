use std::collections::HashSet;

use crate::value_objects::{
    artist_id::ArtistId, image_cover::ImageCover, provider_url::ProviderUrl,
};

pub struct Artist {
    ids: HashSet<ArtistId>,
    name: String,
    images: HashSet<ImageCover>,
    provider_urls: HashSet<ProviderUrl>,
}

impl Artist {
    pub fn new(
        ids: HashSet<ArtistId>,
        name: String,
        images: HashSet<ImageCover>,
        provider_urls: HashSet<ProviderUrl>,
    ) -> Self {
        Self {
            ids,
            name,
            images,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashSet<ArtistId> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn images(&self) -> &HashSet<ImageCover> {
        &self.images
    }

    pub fn provider_urls(&self) -> &HashSet<ProviderUrl> {
        &self.provider_urls
    }
}
