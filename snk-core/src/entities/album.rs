use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use url::Url;

use crate::value_objects::{
    image_cover::ImageCover, product_id::ProductId, provider::provider_id::ProviderId,
};

pub struct Album {
    ids: HashSet<ProductId>,
    name: String,
    release_date: DateTime<Utc>,
    covers: HashSet<ImageCover>,
    provider_urls: HashMap<ProviderId, Url>,
}

impl Album {
    pub fn new(
        ids: HashSet<ProductId>,
        name: String,
        release_date: DateTime<Utc>,
        covers: HashSet<ImageCover>,
        provider_urls: HashMap<ProviderId, Url>,
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

    pub fn provider_urls(&self) -> &HashMap<ProviderId, Url> {
        &self.provider_urls
    }
}
