use std::ops::Deref;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ProviderId {
    id: String,
}

impl ProviderId {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn value(&self) -> String {
        self.id.clone()
    }
}

impl Deref for ProviderId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
