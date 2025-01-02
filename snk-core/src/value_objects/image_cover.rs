use url::Url;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ImageCover {
    Sm(Url),
    Md(Url),
    Lg(Url),
    Default(Url),
    Other(Url),
}

impl ImageCover {
    pub fn url(&self) -> &Url {
        match self {
            ImageCover::Sm(url) => url,
            ImageCover::Md(url) => url,
            ImageCover::Lg(url) => url,
            ImageCover::Default(url) => url,
            ImageCover::Other(url) => url,
        }
    }
}
