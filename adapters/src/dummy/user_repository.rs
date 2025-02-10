use chrono::DateTime;
use snk_core::{
    contracts::repositories::user_repository::{UserRepository, UserRepositoryResult},
    entities::user::User,
};
use uuid::Uuid;

pub struct DummyUserRepository {}

impl UserRepository for DummyUserRepository {
    async fn add(&self, user: User) -> UserRepositoryResult<User> {
        Ok(user)
    }

    async fn update(&self, user: User) -> UserRepositoryResult<User> {
        Ok(user)
    }

    async fn get(&self, user_id: Uuid) -> UserRepositoryResult<Option<User>> {
        Ok(Some(User::new(
            user_id,
            String::from("dummy@test.test"),
            true,
            String::from("hashed_password"),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
            DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
                .unwrap()
                .into(),
        )))
    }

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>> {
        Ok(vec![User::new(
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
        )])
    }

    async fn delete(&self, user: User) -> UserRepositoryResult<User> {
        Ok(user)
    }
}
