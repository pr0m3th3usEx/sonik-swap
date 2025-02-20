#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProviderName {
  name: String,
}

impl ProviderName {
  pub fn new<T: ToString>(name: T) -> Self {
    Self { name: name.to_string() }
  }

  pub fn value(&self) -> String {
    self.name.clone()
  }
}