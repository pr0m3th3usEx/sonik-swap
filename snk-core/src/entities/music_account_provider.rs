use url::Url;
use uuid::Uuid;

#[derive(Hash)]
pub struct MusicAccountProvider {
    id: Uuid,                           // Sonik Swap Provider ID
    name: String,                       // Name of the platform
    color: String,                      // # Hex decimal color
    base_url: Url,                      // OAuth2 Base URL
    token_url: Url,                     // OAuth2 Token URL
    authorizations_needed: Vec<String>, // Authorization needed for OAuth scope (Ex: manage_library)
}

impl MusicAccountProvider {
    pub fn new(
        id: Uuid,
        name: String,
        color: String,
        base_url: Url,
        token_url: Url,
        authorizations_needed: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            color,
            base_url,
            token_url,
            authorizations_needed,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn color(&self) -> &String {
        &self.color
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn token_url(&self) -> &Url {
        &self.token_url
    }

    pub fn authorizations_needed(&self) -> &Vec<String> {
        &self.authorizations_needed
    }
}
