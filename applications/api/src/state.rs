use snk_core::contracts::{
    providers::{password_provider::PasswordProvider, user_id_provider::UserIdProvider},
    repositories::user_repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct AppState<UserRepo, UserIdProv, PasswordProv>
where
    UserRepo: UserRepository,
    UserIdProv: UserIdProvider,
    PasswordProv: PasswordProvider,
{
    pub user_repo: UserRepo,
    pub user_id_provider: UserIdProv,
    pub password_provider: PasswordProv,
}
