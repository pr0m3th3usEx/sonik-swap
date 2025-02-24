use core::panic;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use snk_core::{
    contracts::repositories::user_repository::{
        UserRepository, UserRepositoryError, UserRepositoryResult,
    },
    entities::{
        provider_account::ProviderAccount, user::User,
        user_oauth2_external_account::UserOAuth2ExternalAccount,
    },
    value_objects::{misc::email::Email, provider::provider_id::ProviderId, user::user_id::UserId},
};

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<Vec<User>>>,
    users_oauth2_accounts: Arc<RwLock<HashMap<(ProviderId, UserId), UserOAuth2ExternalAccount>>>,
}

impl InMemoryUserRepository {
    pub async fn fake_oauth2_link(
        &self,
        user_id: &UserId,
        provider_id: &ProviderId,
        provider_account_info: ProviderAccount,
    ) {
        let mut users_oauth2_accounts_store =
            self.users_oauth2_accounts.write().expect("lock poisoned");

        let Some(external_account_info) =
            users_oauth2_accounts_store.get_mut(&(provider_id.clone(), *user_id))
        else {
            panic!("Add a user first !");
        };

        external_account_info.provider_account_info = Some(provider_account_info);
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        InMemoryUserRepository {
            users: Arc::new(RwLock::new(Vec::new())),
            users_oauth2_accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn add(&self, user: User) -> UserRepositoryResult<User> {
        let mut user_store: std::sync::RwLockWriteGuard<'_, Vec<User>> =
            self.users.write().expect("lock poisoned");
        let mut users_oauth2_accounts_store =
            self.users_oauth2_accounts.write().expect("lock poisoned");

        if user_store.iter().any(|u| u.email == user.email) {
            return Err(UserRepositoryError::ServiceError(
                "User already exists".to_string(),
            ));
        }

        user_store.push(user.clone());

        // Add external oauth accounts default (only spotify for test)
        let spotify_provider_id = ProviderId::new("spotify".to_string());
        users_oauth2_accounts_store.insert(
            (spotify_provider_id.clone(), user.id),
            UserOAuth2ExternalAccount::new(spotify_provider_id, user.id, None),
        );

        Ok(user)
    }

    async fn update(&self, _old: User, new: User) -> UserRepositoryResult<User> {
        let mut store = self.users.write().expect("lock poisoned");

        let result = store.iter_mut().find(|u| u.id == new.id);

        if let Some(user) = result {
            *user = new.clone();
            Ok(new)
        } else {
            Err(UserRepositoryError::ServiceError(
                "User not found".to_string(),
            ))
        }
    }

    async fn get(&self, user_id: &UserId) -> UserRepositoryResult<Option<User>> {
        let store = self.users.read().expect("lock poisoned");

        Ok(store.iter().find(|u| u.id == *user_id).cloned())
    }

    async fn get_from_email(&self, email: &Email) -> UserRepositoryResult<Option<User>> {
        let store = self.users.read().expect("lock poisoned");

        Ok(store.iter().find(|u| u.email == *email).cloned())
    }

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>> {
        let store = self.users.read().expect("lock poisoned");

        Ok(store.clone())
    }

    async fn delete(&self, user_id: &UserId) -> UserRepositoryResult<User> {
        let mut store = self.users.write().expect("lock poisoned");

        let index = store.iter().position(|u| u.id == *user_id);

        match index {
            Some(index) => {
                let user = store.remove(index);
                Ok(user)
            }
            None => Err(UserRepositoryError::ServiceError(
                "User not found".to_string(),
            )),
        }
    }

    async fn get_user_provider_account_id(
        &self,
        provider_id: &ProviderId,
        external_account_info: &ProviderAccount,
    ) -> UserRepositoryResult<Option<User>> {
        let store = self.users.read().expect("lock poisoned");
        let user_oauth2_external_account_store =
            self.users_oauth2_accounts.read().expect("lock poisoned");
        let mut found_user_id: Option<UserId> = None;

        for (key, accounts_info) in user_oauth2_external_account_store.iter() {
            if key.0 != *provider_id {
                continue;
            }

            if let Some(info) = &accounts_info.provider_account_info {
                if info.account_id == external_account_info.account_id {
                    found_user_id = Some(accounts_info.user_id);
                    break;
                }
            }
        }

        let Some(user_id) = found_user_id else {
            return Ok(None);
        };

        Ok(store.iter().find(|u| u.id == user_id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use snk_core::{
        contracts::repositories::user_repository::UserRepository,
        entities::{provider_account::ProviderAccount, user::User},
        value_objects::{
            misc::{date::Date, email::Email},
            provider::provider_id::ProviderId,
            provider_account::{
                provider_account_id::ProviderAccountId,
                provider_account_username::ProviderAccountUsername,
            },
            user::{user_id::UserId, user_password::UserPassword},
        },
    };
    use uuid::Uuid;

    use crate::in_memory::user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_add_user() {
        // Arrange
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let result = repository.add(user.clone()).await.unwrap();

        assert_eq!(result, user);
    }

    #[tokio::test]
    async fn test_get_user() {
        // Arrange
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.get(&user.id).await.unwrap();

        assert_eq!(result, Some(user));
    }

    #[tokio::test]
    async fn test_get_user_from_email() {
        // Arrange
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository
            .get_from_email(&Email::new("dummy@test.test").unwrap())
            .await
            .unwrap();

        assert_eq!(result, Some(user));
    }

    #[tokio::test]
    async fn test_get_user_provider_account_id() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let spotify_provider_id = ProviderId::new("spotify".to_string());
        let spotify_provider_account_info = ProviderAccount {
            account_id: ProviderAccountId::new("testtest123".to_string()),
            username: ProviderAccountUsername::new("dahyun@twice.kr".to_string()),
        };

        let _ = repository.add(user.clone()).await.unwrap();

        repository
            .fake_oauth2_link(
                user.id(),
                &spotify_provider_id,
                spotify_provider_account_info.clone(),
            )
            .await;

        // Get from external account
        let result = repository
            .get_user_provider_account_id(&spotify_provider_id, &spotify_provider_account_info)
            .await
            .expect("should be ok");

        assert!(result.is_some());
        assert_eq!(result.unwrap().id(), user.id());
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.delete(&user.id).await.unwrap();

        assert_eq!(result, user);
        assert_eq!(repository.get_all().await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.get_all().await.unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_update_user() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            Email::new("dummy@test.test").unwrap(),
            true,
            UserPassword::from_hash("hashed_password"),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
            Some(
                Date::new(Into::<DateTime<Utc>>::into(
                    DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00").unwrap(),
                ))
                .unwrap(),
            ),
        );

        let _ = repository.add(user.clone()).await.unwrap();

        let mut new = user.clone();
        new.updated_at = Some(
            Date::new(Into::<DateTime<Utc>>::into(
                DateTime::parse_from_rfc3339("2020-05-12T22:10:57+02:00").unwrap(),
            ))
            .unwrap(),
        );

        let result = repository.update(user.clone(), new.clone()).await.unwrap();

        assert_eq!(result, new);
    }
}
