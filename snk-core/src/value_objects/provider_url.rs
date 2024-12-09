use url::Url;

#[derive(Hash)]
pub enum ProviderUrl {
    Spotify(Url),
    Deezer(Url),
}

impl ProviderUrl {
    pub fn url(&self) -> &Url {
        match self {
            ProviderUrl::Spotify(url) => url,
            ProviderUrl::Deezer(url) => url,
        }
    }
}
