#[derive(Debug, Clone)]
pub enum PlaylistId {
    LikedSongs,
    Owned(String),
}
