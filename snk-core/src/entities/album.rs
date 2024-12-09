use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::value_objects::{
    image_cover::ImageCover, product_id::ProductId, provider_url::ProviderUrl,
};

pub struct Album {
    ids: HashSet<ProductId>,
    name: String,
    release_date: DateTime<Utc>,
    covers: HashSet<ImageCover>,
    provider_urls: HashSet<ProviderUrl>,
}

impl Album {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        release_date: DateTime<Utc>,
        covers: HashSet<ImageCover>,
        provider_urls: HashSet<ProviderUrl>,
    ) -> Self {
        Self {
            ids,
            name,
            release_date,
            covers,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashSet<ProductId> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn release_date(&self) -> &DateTime<Utc> {
        &self.release_date
    }

    pub fn covers(&self) -> &HashSet<ImageCover> {
        &self.covers
    }

    pub fn provider_urls(&self) -> &HashSet<ProviderUrl> {
        &self.provider_urls
    }
}
