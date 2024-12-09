use super::provider::Provider;

#[derive(Hash)]
pub enum ProductId {
    ISRC(String),
    UPC(String),
    EAN(String),
    Provider((Provider, String))
}

impl ProductId {
    pub fn id(&self) -> &String {
        match self {
            ProductId::ISRC(id) => id,
            ProductId::UPC(id) => id,
            ProductId::EAN(id) => id,
            ProductId::Provider((_, id)) => id,
        }
    }
}
