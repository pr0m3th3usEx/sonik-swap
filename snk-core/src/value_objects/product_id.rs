use super::provider::provider_id::ProviderId;

#[derive(Hash, PartialEq, Eq)]
pub enum ProductId {
    ISRC(String),
    UPC(String),
    EAN(String),
    Provider((ProviderId, String)),
}

impl ProductId {
    pub fn id(&self) -> &String {
        match self {
            ProductId::ISRC(id) => id,
            ProductId::UPC(id) => id,
            ProductId::EAN(id) => id,
            ProductId::Provider((_, id)) => &id,
        }
    }
}
