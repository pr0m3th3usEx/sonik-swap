use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum PlaylistId {
    LikedSongs,
    Owned(String),
}

impl Display for PlaylistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlaylistId::LikedSongs => "favourites",
                PlaylistId::Owned(name) => name,
            }
        )
    }
}
