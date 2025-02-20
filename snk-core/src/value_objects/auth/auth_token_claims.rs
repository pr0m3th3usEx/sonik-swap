use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthTokenClaims {
    sub: String,
    exp: i64,
    once: i64,
}

impl AuthTokenClaims {
    pub fn new(sub: String, exp: i64, once: i64) -> Self {
        Self { sub, exp, once }
    }
}
