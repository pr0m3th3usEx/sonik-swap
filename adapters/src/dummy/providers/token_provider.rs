use snk_core::contracts::providers::token_provider::{TokenProvider, TokenProviderResult};

pub struct DummyTokenProvider {}

impl TokenProvider for DummyTokenProvider {
    async fn generate_token<T>(&self, _claims: T) -> TokenProviderResult<String>
    where
        T: serde::Serialize,
    {
        Ok("fake_token".to_string())
    }

    async fn verify_token<T: Default>(&self, _token: &str) -> TokenProviderResult<T> {
        Ok(T::default())
    }
}
