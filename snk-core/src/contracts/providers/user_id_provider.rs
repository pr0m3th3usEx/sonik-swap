use uuid::Uuid;

pub trait UserIdProvider {
  fn generate(&self) -> Uuid;
}
