#[derive(Hash)]
pub enum ProductId {
    ISRC(String),
    UPC(String),
    EAN(String),
    Spotify(String),
    Deezer(String),
}

impl ProductId {
    pub fn id(&self) -> &String {
        match self {
            ProductId::ISRC(id) => id,
            ProductId::UPC(id) => id,
            ProductId::EAN(id) => id,
            ProductId::Spotify(id) => id,
            ProductId::Deezer(id) => id,
        }
    }
}
