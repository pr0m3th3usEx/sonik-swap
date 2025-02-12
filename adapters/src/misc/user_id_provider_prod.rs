use snk_core::contracts::providers::user_id_provider::UserIdProvider;
use uuid::Uuid;

#[derive(Default)]
pub struct UserIdProviderProd {}

impl UserIdProviderProd {
  pub fn new() -> Self {
    Default::default()
  }
}

impl UserIdProvider for UserIdProviderProd {
  fn generate(&self) -> Uuid {
    Uuid::new_v4()
  }
}