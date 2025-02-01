use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: Uuid,
        username: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
            created_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
