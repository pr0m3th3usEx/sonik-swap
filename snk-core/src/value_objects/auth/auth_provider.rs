use std::str::FromStr;

pub enum AuthProvider {
    Spotify,
    Deezer,
}

impl FromStr for AuthProvider {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "spotify" => Ok(AuthProvider::Spotify),
            "deezer" => Ok(AuthProvider::Deezer),
            _ => Err("unknown provider")
        }
    }
}