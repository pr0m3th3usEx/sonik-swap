use std::sync::{Arc, RwLock};

use snk_core::{contracts::repositories::user_repository::{UserRepository, UserRepositoryError, UserRepositoryResult}, entities::user::User};
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<Vec<User>>>,
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        InMemoryUserRepository {
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn add(&self, user: User) -> UserRepositoryResult<User> {
        let mut store = self.users.write().expect("lock poisoned");

        if store.iter().any(|u| u.email == user.email) {
            return Err(UserRepositoryError::ServiceError("User already exists".to_string()));
        }

        store.push(user.clone());
        Ok(user)
    }

    async fn update(&self, _old: User, new: User) -> UserRepositoryResult<User> {
        let mut store = self.users.write().expect("lock poisoned");

        let result = store.iter_mut().find(|u| u.id == new.id);
        
        if let Some(user) = result {
            *user = new.clone();
            Ok(new)
        } else {
            Err(UserRepositoryError::ServiceError("User not found".to_string()))
        }
    }

    async fn get(&self, user_id: Uuid) -> UserRepositoryResult<Option<User>> {
        let store = self.users.read().expect("lock poisoned");

        Ok(store.iter().find(|u| u.id == user_id).cloned())
    }

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>> {
        let store = self.users.read().expect("lock poisoned");

        Ok(store.clone())
    }

    async fn delete(&self, user_id: Uuid) -> UserRepositoryResult<User> {
        let mut store = self.users.write().expect("lock poisoned");

        let index = store.iter().position(|u| u.id == user_id);

        match index {
            Some(index) => {
                let user = store.remove(index);
                Ok(user)
            },
            None => Err(UserRepositoryError::ServiceError("User not found".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use snk_core::{contracts::repositories::user_repository::UserRepository, entities::user::User};
    use uuid::Uuid;

    use crate::in_memory::user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_add_user() {
        // Arrange
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            Uuid::new_v4(),
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        );

        let result = repository.add(user.clone()).await.unwrap();

        assert_eq!(result, user);
    }

    #[tokio::test]
    async fn test_get_user() {
        // Arrange
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            Uuid::new_v4(),
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.get(user.id).await.unwrap();

        assert_eq!(result, Some(user));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            Uuid::new_v4(),
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.delete(user.id).await.unwrap();

        assert_eq!(result, user);
        assert_eq!(repository.get_all().await.unwrap().len(), 0);       
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            Uuid::new_v4(),
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        );

        let _ = repository.add(user.clone()).await.unwrap();
        let result = repository.get_all().await.unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_update_user() {
        let repository = InMemoryUserRepository::default();
        let user = User::new(
            Uuid::new_v4(),
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        );

        let _ = repository.add(user.clone()).await.unwrap();

        let mut new = user.clone();
        new.updated_at = DateTime::parse_from_rfc3339("2020-05-12T23:10:57+02:00")
            .unwrap()
            .into();

        let result = repository.update(user.clone(), new.clone()).await.unwrap();

        assert_eq!(result, new);
    }
}
