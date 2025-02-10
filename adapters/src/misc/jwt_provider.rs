use jsonwebtoken::{errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use snk_core::contracts::providers::token_provider::{TokenProvider, TokenProviderError, TokenProviderResult};

pub struct JwtProvider {
    secret: String,
}

impl JwtProvider {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenProvider for JwtProvider {
    async fn generate_token<T>(
        &self,
        claims: T,
    ) -> TokenProviderResult<String>
    where
        T: serde::Serialize,
    {
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_base64_secret(&self.secret)
                .map_err(|e| TokenProviderError::InternalError(e.to_string()))?,
        )
        .map_err(|e| TokenProviderError::InternalError(e.to_string()))
    }

    async fn verify_token<T>(
        &self,
        token: &str,
    ) -> TokenProviderResult<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let result = jsonwebtoken::decode::<T>(
            token,
            &DecodingKey::from_base64_secret(&self.secret)
                .map_err(|e| TokenProviderError::InternalError(e.to_string()))?,
            &Validation::default(),
        );

        match result {
            Ok(claims) => Ok(claims.claims),
            Err(e) => match e.kind() {
                ErrorKind::ExpiredSignature => Err(TokenProviderError::ExpiredToken),
                _ => Err(TokenProviderError::InternalError(e.to_string())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    #[tokio::test]
    async fn test_generate_token() {
        // c2VjcmV0 => secret in base64
        let provider = JwtProvider::new("c2VjcmV0".to_string());
        let claims = Claims {
            sub: "test".to_string(),
            exp: 1000,
        };

        let token = provider.generate_token(claims).await.unwrap();
        assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0IiwiZXhwIjoxMDAwfQ.m4QC1TOzje6XUI0-C2ANzUOnOHf4UQUIAqwXfsWInnw");  
    }

    #[tokio::test]
    async fn test_verify_token() {
        // c2VjcmV0 => secret in base64
        let provider = JwtProvider::new("c2VjcmV0".to_string());
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0IiwiZXhwIjoxNzcwNzI0MTYyMzIxfQ.pfQF7p54nuRWh6qSSZKa-eoc4By6qLnGPyJm1Yi6og8";
        
        let claims  = provider.verify_token::<Claims>(token).await.unwrap();

        assert_eq!(claims.sub, "test");
        assert_eq!(claims.exp, 1770724162321);
    }
}