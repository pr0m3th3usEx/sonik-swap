use snk_core::contracts::{
    providers::{
        password_provider::PasswordProvider, token_provider::TokenProvider,
        user_id_provider::UserIdProvider,
    },
    repositories::user_repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct AppState<UserRepo, UserIdProv, PasswordProv, AccessTokenProv, RefreshTokenProv>
where
    UserRepo: UserRepository,
    UserIdProv: UserIdProvider,
    PasswordProv: PasswordProvider,
    AccessTokenProv: TokenProvider,
    RefreshTokenProv: TokenProvider,
{
    pub user_repo: UserRepo,
    pub user_id_provider: UserIdProv,
    pub password_provider: PasswordProv,
    pub access_token_provider: AccessTokenProv,
    pub refresh_token_provider: RefreshTokenProv,
}
