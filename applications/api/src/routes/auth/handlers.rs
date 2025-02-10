use axum::response::IntoResponse;
use snk_core::contracts::repositories::{
    email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
};

/// - POST /auth/signup: Create a new user account
pub async fn signup<UserRepo, EmailVerificationRepo>() -> impl IntoResponse
where
    UserRepo: UserRepository,
    EmailVerificationRepo: EmailVerificationRepository,
{
    "signup".to_string()
}

/// - POST /auth/login: Login user
pub async fn login<UserRepo, EmailVerificationRepo>() -> impl IntoResponse
where
    UserRepo: UserRepository,
    EmailVerificationRepo: EmailVerificationRepository,
{
    "login".to_string()
}

// TODO
// - GET  /auth/{providerId}: Return OAuth2 authentication URL + ?=state=providerId
// pub async fn oauth2<UserRepo, EmailVerificationRepo>() -> impl IntoResponse
// where
//     UserRepo: UserRepository,
//     EmailVerificationRepo: EmailVerificationRepository,
// {
//     "Sign up with oauth2".to_string()
// }

// TODO
// - GET  /auth/oauth2/signup/callback: FindOrCreate user account
// pub async fn on_oauth2_callback<UserRepo, EmailVerificationRepo>() -> impl IntoResponse
// where
//     UserRepo: UserRepository,
//     EmailVerificationRepo: EmailVerificationRepository,
// {
//     "Sign up callback for oauth2".to_string()
// }
