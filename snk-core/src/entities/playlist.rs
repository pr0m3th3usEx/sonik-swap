use std::collections::HashSet;

use url::Url;

use crate::value_objects::{image_cover::ImageCover, playlist_id::PlaylistId, provider::provider_id::ProviderId};

pub struct Playlist {
    id: PlaylistId,
    name: String,
    covers: HashSet<ImageCover>,
    owner: String, // Name of the owner (We won't use other metadata for now)
    provider_id: ProviderId,
    provider_url: Url,
    total_songs: u32,
}

impl Playlist {
    pub fn new(
        id: PlaylistId,
        name: String,
        covers: HashSet<ImageCover>,
        owner: String,
        total_songs: u32,
        provider_id: ProviderId,
        provider_url: Url,
    ) -> Self {
        Self {
            id,
            name,
            covers,
            owner,
            total_songs,
            provider_id,
            provider_url,
        }
    }

    pub fn id(&self) -> &PlaylistId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn covers(&self) -> &HashSet<ImageCover> {
        &self.covers
    }

    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn total_songs(&self) -> u32 {
        self.total_songs
    }

    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    pub fn provider_url(&self) -> &Url {
        &self.provider_url
    }
}
