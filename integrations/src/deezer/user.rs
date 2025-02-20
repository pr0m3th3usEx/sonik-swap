use serde::Deserialize;
use snk_core::{
    entities::provider_account::ProviderAccount,
    value_objects::provider_account::{
        provider_account_id::ProviderAccountId, provider_account_username::ProviderAccountUsername,
    },
};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct DeezerUser {
    pub id: String,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    pub status: Option<u64>,
    pub birthday: Option<String>,
    pub inscription_date: Option<String>,
    pub gender: Option<String>,
    pub link: Option<Url>,
    // The url of the playlist's cover. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    pub picture_small: Option<Url>,
    pub picture_medium: Option<Url>,
    pub picture_big: Option<Url>,
    pub picture_xl: Option<Url>,
    pub country: String,
    pub lang: String,
    pub is_kid: bool,
    pub explicit_content_level: Option<String>,
    pub explicit_content_levels_available: Vec<String>,
    pub tracklist: Option<Url>,
}

impl Into<ProviderAccount> for DeezerUser {
    fn into(self) -> ProviderAccount {
        let account_id = ProviderAccountId::new(self.id);
        let username = ProviderAccountUsername::new(self.email);

        ProviderAccount {
            account_id,
            username,
        }
    }
}
