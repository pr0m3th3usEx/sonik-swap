pub enum ArtistId {
    Spotify(String),
    Deezer(String),
}

impl ArtistId {
    pub fn id(&self) -> &String {
        match self {
            ArtistId::Spotify(id) => id,
            ArtistId::Deezer(id) => id,
        }
    }
}
