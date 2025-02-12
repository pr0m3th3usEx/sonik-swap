use snk_core::{contracts::providers::user_id_provider::UserIdProvider, value_objects::user::user_id::UserId};
use uuid::Uuid;

#[derive(Default)]
pub struct UserIdProviderProd {}

impl UserIdProviderProd {
  pub fn new() -> Self {
    Default::default()
  }
}

impl UserIdProvider for UserIdProviderProd {
  fn generate(&self) -> UserId {
    UserId::new(Uuid::new_v4()).expect("bad uuid")
  }
}