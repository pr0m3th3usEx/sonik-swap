use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand_core::OsRng;
use snk_core::contracts::providers::password_provider::{
    PasswordProvider, PasswordProviderError, PasswordProviderResult,
};

#[derive(Clone)]
pub struct PasswordProviderProd<'a> {
    engine: Argon2<'a>,
    salt_string: SaltString,
}

impl PasswordProviderProd<'_> {
    pub fn new(salt_string: Option<SaltString>) -> Self {
        let salt_string = salt_string.unwrap_or(SaltString::generate(&mut OsRng));

        Self {
            salt_string,
            engine: Argon2::default(),
        }
    }
}

impl Default for PasswordProviderProd<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl PasswordProvider for PasswordProviderProd<'_> {
    async fn hash_password(&self, password: &str) -> PasswordProviderResult<String> {
        let hash_password = self
            .engine
            .hash_password(password.as_bytes(), &self.salt_string)
            .map_err(|e| PasswordProviderError::HashError(e.to_string()))?;

        Ok(hash_password.to_string())
    }

    async fn verify_password(&self, password: &str, hash: &str) -> PasswordProviderResult<bool> {
        let passowrd_hash = PasswordHash::new(hash)
            .map_err(|e| PasswordProviderError::VerifyError(e.to_string()))?;

        if let Err(e) = self
            .engine
            .verify_password(password.as_bytes(), &passowrd_hash)
        {
            return match e {
                password_hash::Error::Password => Ok(false),
                other => Err(PasswordProviderError::VerifyError(other.to_string())),
            };
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::SaltString;
    use snk_core::contracts::providers::password_provider::PasswordProvider;

    use super::PasswordProviderProd;

    #[tokio::test]
    async fn test_hash_password() {
        let pwd_provider = PasswordProviderProd::new(Some(
            SaltString::from_b64("YmFkIHNhbHQh").expect("valid base64 salt"),
        ));
        let password = "password";
        let expected = "$argon2id$v=19$m=19456,t=2,p=1$YmFkIHNhbHQh$DqHGwv6NQV0VcaJi7jeF1E8IpfMXmXcpq4r2kKyqpXk";

        let result = pwd_provider
            .hash_password(password)
            .await
            .expect("valid hash");

        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn test_verify_password() {
        let pwd_provider = PasswordProviderProd::new(None);

        let password = "password";
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$YmFkIHNhbHQh$DqHGwv6NQV0VcaJi7jeF1E8IpfMXmXcpq4r2kKyqpXk";

        let is_valid = pwd_provider
            .verify_password(password, hash)
            .await
            .expect("no error");

        assert!(is_valid);
    }
}
