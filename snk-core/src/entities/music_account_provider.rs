use oauth2::{AuthUrl, Scope, TokenUrl};

use crate::value_objects::provider::{provider_id::ProviderId, provider_name::ProviderName};

#[derive(Clone, Hash)]
pub struct MusicAccountProvider {
    /// Sonik Swap Provider ID
    pub id: ProviderId,                    
    /// Name of the platform
    pub name: ProviderName,  
    /// Hex decimal color              
    pub color: u32,             
    /// OAuth2 Authorization URL          
    pub auth_url: AuthUrl,
    /// OAuth2 Token URL                 
    pub token_url: TokenUrl,
    /// Usage for account creation & authentication            
    pub authentication_allowed: bool,
    /// Authorization needed for OAuth scope (Ex: manage_library)
    pub authorizations_needed: Vec<Scope>,
}

impl MusicAccountProvider {
    pub fn new(
        id: ProviderId,
        name: ProviderName,
        color: u32,
        auth_url: AuthUrl,
        token_url: TokenUrl,
        authentication_allowed: bool,
        authorizations_needed: Vec<Scope>,
    ) -> Self {
        Self {
            id,
            name,
            color,
            auth_url,
            token_url,
            authentication_allowed,
            authorizations_needed,
        }
    }

    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    pub fn name(&self) -> &ProviderName {
        &self.name
    }

    pub fn color(&self) -> u32 {
        self.color
    }

    pub fn auth_url(&self) -> &AuthUrl {
        &self.auth_url
    }

    pub fn token_url(&self) -> &TokenUrl {
        &self.token_url
    }

    pub fn authentication_allowed(&self) -> bool {
        self.authentication_allowed
    }

    pub fn authorizations_needed(&self) -> &Vec<Scope> {
        &self.authorizations_needed
    }
}
