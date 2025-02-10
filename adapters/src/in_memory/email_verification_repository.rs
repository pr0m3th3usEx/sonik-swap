use std::sync::{Arc, RwLock};

use snk_core::{
    contracts::repositories::email_verification_repository::{
        EmailVerificationRepository, EmailVerificationRepositoryError, EmailVerificationRepositoryResult
    },
    entities::email_verification::EmailVerificationToken,
};
#[derive(Clone)]
pub struct InMemoryEmailVerificationRepository {
    email_tokens: Arc<RwLock<Vec<EmailVerificationToken>>>,
}

impl Default for InMemoryEmailVerificationRepository {
    fn default() -> Self {
        InMemoryEmailVerificationRepository {
            email_tokens: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl EmailVerificationRepository for InMemoryEmailVerificationRepository {
    async fn add(
        &self, ev_token: EmailVerificationToken
    ) -> EmailVerificationRepositoryResult<EmailVerificationToken> {
        let mut store = self.email_tokens.write().expect("lock poisoned");

        store.push(ev_token.clone());

        Ok(ev_token)
    }

    async fn get(
        &self,
        user_id: &str,
        token: &str,
    ) -> EmailVerificationRepositoryResult<Option<EmailVerificationToken>> {
        let store = self.email_tokens.read().expect("lock poisoned");

        let result = store.iter().find(|ev_token| {
            ev_token.user_id == user_id && ev_token.token == token
        });

        Ok(result.cloned())
    }
    
    async fn update(&self, old: EmailVerificationToken, new: EmailVerificationToken) -> EmailVerificationRepositoryResult<EmailVerificationToken> {
        let mut store = self.email_tokens.write().expect("lock poisoned");

        let result = store.iter_mut().find(|ev_token| {
            ev_token.user_id == old.user_id && ev_token.token == old.token
        });

        if let Some(ev_token) = result {
            *ev_token = new;
            Ok(ev_token.clone())
        } else {
            Err(EmailVerificationRepositoryError::ServiceError("Token not found".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use snk_core::entities::email_verification::EmailVerificationToken;

    #[tokio::test]
    async fn test_add() {
        let repo = InMemoryEmailVerificationRepository::default();

        let user_id = "user_id".to_string();
        let token = "token".to_string();
        let expires_at = Utc::now();
        let created_at = Utc::now();

        let ev_token = EmailVerificationToken::new(
            user_id,
            token,
            false,
            expires_at,
            created_at
        );

        let result = repo.add(ev_token.clone()).await.unwrap();

        assert_eq!(ev_token, result);
    }

    #[tokio::test]
    async fn test_get() {
        let repo = InMemoryEmailVerificationRepository::default();

        let user_id = "user_id".to_string();
        let token = "token".to_string();
        let expires_at = Utc::now();
        let created_at = Utc::now();

        let ev_token = EmailVerificationToken::new(
            user_id,
            token,
            false,
            expires_at,
            created_at
        );


        repo.add(ev_token.clone()).await.unwrap();

        let result = repo.get("user_id", "token").await.unwrap();

        assert_eq!(Some(ev_token), result);
    }

    #[tokio::test]
    async fn test_update() {
        let repo = InMemoryEmailVerificationRepository::default();

        let user_id = "user_id".to_string();
        let token = "token".to_string();
        let expires_at = Utc::now();
        let created_at = Utc::now();

        let ev_token = EmailVerificationToken::new(
            user_id.clone(),
            token.clone(),
            false,
            expires_at,
            created_at,
        );

        repo.add(ev_token.clone()).await.unwrap();

        let new_ev_token = EmailVerificationToken::new(
            user_id,
            token,
            true,
            Utc::now(),
            created_at
        );

        let result = repo.update(ev_token.clone(), new_ev_token.clone()).await.unwrap();

        assert_eq!(new_ev_token, result);
    }
}