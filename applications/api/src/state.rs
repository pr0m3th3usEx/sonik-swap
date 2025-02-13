use snk_core::contracts::repositories::user_repository::UserRepository;

#[derive(Debug, Clone)]
pub struct AppState<UserRepo>
where
    UserRepo: UserRepository,
{
    pub user_repo: UserRepo,
}
