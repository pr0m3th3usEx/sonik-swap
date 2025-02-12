use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        email_verified: bool,
        password: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            email,
            email_verified,
            password,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn created_at(&self) -> &Option<DateTime<Utc>> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
        &self.updated_at
    }
}
