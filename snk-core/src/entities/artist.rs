use std::collections::{HashMap, HashSet};

use url::Url;

use crate::value_objects::{image_cover::ImageCover, provider::provider_id::ProviderId};

#[derive(PartialEq, Eq)]
pub struct Artist {
    ids: HashMap<ProviderId, String>,
    name: String,
    images: HashSet<ImageCover>,
    provider_urls: HashMap<ProviderId, Url>,
}

impl Artist {
    pub fn new(
        ids: HashMap<ProviderId, String>,
        name: String,
        images: HashSet<ImageCover>,
        provider_urls: HashMap<ProviderId, Url>,
    ) -> Self {
        Self {
            ids,
            name,
            images,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashMap<ProviderId, String> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn images(&self) -> &HashSet<ImageCover> {
        &self.images
    }

    pub fn provider_urls(&self) -> &HashMap<ProviderId, Url> {
        &self.provider_urls
    }
}
