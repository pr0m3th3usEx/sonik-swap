use crate::value_objects::provider_account::{
    provider_account_id::ProviderAccountId, provider_account_username::ProviderAccountUsername,
};

#[derive(Debug, Clone)]
pub struct ProviderAccount {
    pub account_id: ProviderAccountId,
    pub username: ProviderAccountUsername,
}
