#[derive(Debug, Hash, Clone)]
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
