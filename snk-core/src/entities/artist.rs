use std::collections::HashMap;

use url::Url;

use crate::value_objects::provider::provider_id::ProviderId;

#[derive(PartialEq, Eq)]
pub struct Artist {
    pub ids: HashMap<ProviderId, String>,
    pub name: String,
    pub provider_urls: HashMap<ProviderId, Url>,
}

impl Artist {
    pub fn new(
        ids: HashMap<ProviderId, String>,
        name: String,
        provider_urls: HashMap<ProviderId, Url>,
    ) -> Self {
        Self {
            ids,
            name,
            provider_urls,
        }
    }

    pub fn ids(&self) -> &HashMap<ProviderId, String> {
        &self.ids
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn provider_urls(&self) -> &HashMap<ProviderId, Url> {
        &self.provider_urls
    }
}
