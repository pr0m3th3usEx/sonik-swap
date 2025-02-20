use std::ops::Deref;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ProviderAccountId {
    id: String,
}

impl ProviderAccountId {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn value(&self) -> String {
        self.id.clone()
    }
}

impl Deref for ProviderAccountId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
