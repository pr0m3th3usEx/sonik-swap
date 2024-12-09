use chrono::{DateTime, Utc};

pub struct User {
    username: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            username,
            email,
            password,
            created_at,
        }
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
