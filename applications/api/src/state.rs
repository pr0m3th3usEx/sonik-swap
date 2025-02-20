use snk_core::contracts::{
    providers::{
        password_provider::PasswordProvider, token_provider::TokenProvider,
        user_id_provider::UserIdProvider,
    },
    repositories::{
        music_account_provider_repository::MusicAccountProviderRepository,
        user_repository::UserRepository,
    },
};

#[derive(Debug, Clone)]
pub struct AppState<
    UserRepo,
    MusicAccountProvRepo,
    UserIdProv,
    PasswordProv,
    AccessTokenProv,
    RefreshTokenProv,
> where
    UserRepo: UserRepository,
    MusicAccountProvRepo: MusicAccountProviderRepository,
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
    pub music_account_provider_repo: MusicAccountProvRepo,
}
