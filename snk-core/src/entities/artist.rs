use std::collections::{HashMap, HashSet};

use url::Url;

use crate::value_objects::{
    image_cover::ImageCover, provider::Provider
};

pub struct Artist {
    ids: HashMap<Provider, String>,
    name: String,
    images: HashSet<ImageCover>,
    provider_urls: HashMap<Provider, Url>,
}

impl Artist {
    pub fn new(
        ids: HashMap<Provider, String>,
        name: String,
        images: HashSet<ImageCover>,
        provider_urls: HashMap<Provider, Url>,
    ) -> Self {
        Self {
            ids,
            name,
            images,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashMap<Provider, String> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn images(&self) -> &HashSet<ImageCover> {
        &self.images
    }

    pub fn provider_urls(&self) -> &HashMap<Provider, Url> {
        &self.provider_urls
    }
}
