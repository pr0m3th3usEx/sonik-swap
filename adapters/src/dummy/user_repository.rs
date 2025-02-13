use chrono::{DateTime, Utc};
use snk_core::{
    contracts::repositories::user_repository::{UserRepository, UserRepositoryResult},
    entities::user::User,
    value_objects::{
        misc::{date::Date, email::Email},
        user::{user_id::UserId, user_password::UserPassword},
    },
};
use uuid::Uuid;

pub struct DummyUserRepository {}

impl UserRepository for DummyUserRepository {
    async fn add(&self, user: User) -> UserRepositoryResult<User> {
        Ok(user)
    }

    async fn update(&self, _old: User, new: User) -> UserRepositoryResult<User> {
        Ok(new)
    }

    async fn get(&self, user_id: &UserId) -> UserRepositoryResult<Option<User>> {
        Ok(Some(User::new(
            *user_id,
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
        )))
    }

    async fn get_from_email(&self, email: &Email) -> UserRepositoryResult<Option<User>> {
        Ok(Some(User::new(
            UserId::new(Uuid::new_v4()).unwrap(),
            email.clone(),
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
        )))
    }

    async fn get_all(&self) -> UserRepositoryResult<Vec<User>> {
        Ok(vec![User::new(
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
        )])
    }

    async fn delete(&self, user_id: &UserId) -> UserRepositoryResult<User> {
        Ok(User::new(
            *user_id,
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
        ))
    }
}
