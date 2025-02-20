use oauth2::RedirectUrl;
use serde::Deserialize;
use snk_core::value_objects::provider::provider_id::ProviderId;

#[derive(Deserialize)]
pub struct OAuth2AuthorizeRequestParams {
    provider_id: String,
}

#[derive(Debug)]
pub struct OAuth2AuthorizeRequestParamsParsed {
    pub provider_id: ProviderId,
}

impl TryFrom<OAuth2AuthorizeRequestParams> for OAuth2AuthorizeRequestParamsParsed {
    type Error = &'static str;

    fn try_from(value: OAuth2AuthorizeRequestParams) -> Result<Self, Self::Error> {
        let provider_id = ProviderId::new(value.provider_id.to_lowercase());

        Ok(Self { provider_id })
    }
}

#[derive(Deserialize)]
pub struct OAuth2AuthorizeRequestQuery {
    redirect_url: String,
}

#[derive(Debug)]
pub struct OAuth2AuthorizeRequestQueryParsed {
    pub redirect_url: RedirectUrl,
}

impl TryFrom<OAuth2AuthorizeRequestQuery> for OAuth2AuthorizeRequestQueryParsed {
    type Error = &'static str;

    fn try_from(query: OAuth2AuthorizeRequestQuery) -> Result<Self, Self::Error> {
        let Ok(redirect_url) = RedirectUrl::new(query.redirect_url) else {
            return Err("redirect_url");
        };

        Ok(Self { redirect_url })
    }
}
