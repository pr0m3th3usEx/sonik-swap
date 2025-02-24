use chrono::{Duration, Utc};

use crate::{
    contracts::providers::token_provider::{TokenProvider, TokenProviderError},
    value_objects::{auth::auth_token_claims::AuthTokenClaims, user::user_id::UserId},
};

pub async fn generate_token_pair(
    user_id: &UserId,
    access_token_provider: &impl TokenProvider,
    refresh_token_provider: &impl TokenProvider,
) -> Result<(String, String, Duration), TokenProviderError> {
    let access_token_exp = Utc::now() + ACCESS_TOKEN_EXP_TIME;
    let refresh_token_exp = Utc::now() + REFRESH_TOKEN_EXP_TIME;

    let access_token_claims: AuthTokenClaims = AuthTokenClaims::new(
        user_id.value().to_string(),
        access_token_exp.timestamp(),
        0, // TODO once integration
    );

    let refresh_token_claims = AuthTokenClaims::new(
        user_id.value().to_string(),
        refresh_token_exp.timestamp(),
        0, // TODO once integration
    );

    let access_token = access_token_provider
        .generate_token(access_token_claims)
        .await?;
    let refresh_token = refresh_token_provider
        .generate_token(refresh_token_claims)
        .await?;

    Ok((access_token, refresh_token, ACCESS_TOKEN_EXP_TIME))
}

const ACCESS_TOKEN_EXP_TIME: Duration = Duration::days(1);
const REFRESH_TOKEN_EXP_TIME: Duration = Duration::days(7);
