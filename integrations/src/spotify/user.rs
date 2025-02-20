use serde::Deserialize;
use snk_core::{
    entities::provider_account::ProviderAccount,
    value_objects::provider_account::{
        provider_account_id::ProviderAccountId, provider_account_username::ProviderAccountUsername,
    },
};
use url::Url;

use super::common::{SpotifyExternalUrls, SpotifyFollowers, SpotifyImage, SpotifyProduct};

#[derive(Debug, Deserialize)]
pub struct SpotifyUser {
    pub country: String,
    pub display_name: String,
    pub explicit_content: SpotifyUserExplicitContent,
    #[serde(alias = "external_urls")]
    pub _external_urls: SpotifyExternalUrls,
    #[serde(alias = "followers")]
    pub _followers: SpotifyFollowers,
    #[serde(alias = "href")]
    pub _href: Url,
    pub id: String,
    pub email: String,
    #[serde(alias = "images")]
    pub _images: Vec<SpotifyImage>,
    #[serde(alias = "product")]
    pub _product: SpotifyProduct,
    #[serde(alias = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyUserExplicitContent {
    #[serde(alias = "filter_enabled")]
    _filter_enabled: bool,
    #[serde(alias = "filter_locked")]
    _filter_locked: bool,
}

impl Into<ProviderAccount> for SpotifyUser {
    fn into(self) -> ProviderAccount {
        let account_id = ProviderAccountId::new(self.id);
        let username = ProviderAccountUsername::new(self.email);

        ProviderAccount {
            account_id,
            username,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spotify::user::SpotifyUser;

    #[test]
    fn test_deserialize_user() {
        let payload = include_str!("../../tests/spotify/payload_user.json");
        let json = serde_json::from_str::<SpotifyUser>(&payload).expect("valid json");

        assert_eq!(json.id, "darklight956");
        assert_eq!(json.email, "test@gmail.com");
    }
}
