use url::Url;

pub enum ImageCover {
    Sm(Url),
    Md(Url),
    Lg(Url),
    Other(Url),
}

impl ImageCover {
    pub fn url(&self) -> &Url {
        match self {
            ImageCover::Sm(url) => url,
            ImageCover::Md(url) => url,
            ImageCover::Lg(url) => url,
            ImageCover::Other(url) => url,
        }
    }
}
