use std::ops::Deref;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ProviderAccountUsername {
    id: String,
}

impl ProviderAccountUsername {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn value(&self) -> String {
        self.id.clone()
    }
}

impl Deref for ProviderAccountUsername {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
