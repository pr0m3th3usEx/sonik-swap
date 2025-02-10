use snk_core::contracts::repositories::{
    email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct AppState<UserRepo, EmailVerificationRepo>
where
    UserRepo: UserRepository,
    EmailVerificationRepo: EmailVerificationRepository,
{
    pub user_repo: UserRepo,
    pub email_verification_repo: EmailVerificationRepo,
}
