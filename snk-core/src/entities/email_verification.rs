pub struct EmailVerificationToken {
    pub user_id: String,
    pub token: String,
    pub consumed: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl EmailVerificationToken {
    pub fn new(
        user_id: String,
        token: String,
        consumed: bool,
        expires_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            token,
            consumed,
            expires_at,
            created_at,
        }
    }
}
