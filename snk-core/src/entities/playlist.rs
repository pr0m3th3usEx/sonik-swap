use std::collections::HashSet;

use url::Url;

use crate::value_objects::{image_cover::ImageCover, provider::Provider};


pub struct Playlist {
  id: String,
  name: String,
  covers: HashSet<ImageCover>,
  owner: String, // Name of the owner (We won't use other metadata for now)
  provider: Provider,
  provider_url: Url,
  total_songs: u32,
}

impl Playlist {
  pub fn new(id: String, name: String, covers: HashSet<ImageCover>, owner: String, total_songs: u32, provider: Provider, provider_url: Url) -> Self {
    Self {
      id,
      name,
      covers,
      owner,
      total_songs,
      provider,
      provider_url,
    }
  }

  pub fn id(&self) -> &String {
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

  pub fn provider(&self) -> Provider {
    self.provider
  }

  pub fn provider_url(&self) -> &Url {
    &self.provider_url
  }
}

