use crate::value_objects::{provider::provider_id::ProviderId, user::user_id::UserId};

use super::provider_account::ProviderAccount;

#[derive(Debug)]
pub struct UserOAuth2ExternalAccount {
    pub provider_id: ProviderId,
    pub user_id: UserId,
    pub provider_account_info: Option<ProviderAccount>,
}

impl UserOAuth2ExternalAccount {
    pub fn new(
        provider_id: ProviderId,
        user_id: UserId,
        provider_account_info: Option<ProviderAccount>,
    ) -> Self {
        Self {
            provider_id,
            user_id,
            provider_account_info,
        }
    }
}
