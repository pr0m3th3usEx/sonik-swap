use crate::value_objects::{
    misc::{date::Date, email::Email},
    user::{user_id::UserId, user_password::UserPassword},
};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub email_verified: bool,
    pub password: UserPassword,
    pub created_at: Option<Date>,
    pub updated_at: Option<Date>,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        email_verified: bool,
        password: UserPassword,
        created_at: Option<Date>,
        updated_at: Option<Date>,
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

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }

    pub fn created_at(&self) -> &Option<Date> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<Date> {
        &self.updated_at
    }
}
