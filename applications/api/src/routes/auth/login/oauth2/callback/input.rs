use oauth2::{AuthorizationCode, RedirectUrl};
use serde::Deserialize;
use snk_core::value_objects::provider::provider_id::ProviderId;

#[derive(Deserialize)]
pub struct OAuth2CallbackRequestQuery {
    code: String,
    redirect_url: String,
}

#[derive(Debug)]
pub struct OAuth2CallbackRequestQueryParsed {
    pub code: AuthorizationCode,
    pub redirect_url: RedirectUrl,
}

impl TryFrom<OAuth2CallbackRequestQuery> for OAuth2CallbackRequestQueryParsed {
    type Error = &'static str;

    fn try_from(query: OAuth2CallbackRequestQuery) -> Result<Self, Self::Error> {
        let code = AuthorizationCode::new(query.code);
        let Ok(redirect_url) = RedirectUrl::new(query.redirect_url) else {
            return Err("redirect_url");
        };

        Ok(Self { code, redirect_url })
    }
}

#[derive(Deserialize)]
pub struct OAuth2CallbackRequestParams {
    provider_id: String,
}

#[derive(Debug)]
pub struct OAuth2CallbackRequestParamsParsed {
    pub provider_id: ProviderId,
}

impl TryFrom<OAuth2CallbackRequestParams> for OAuth2CallbackRequestParamsParsed {
    type Error = &'static str;

    fn try_from(query: OAuth2CallbackRequestParams) -> Result<Self, Self::Error> {
        let provider_id = ProviderId::new(query.provider_id);

        Ok(Self { provider_id })
    }
}
