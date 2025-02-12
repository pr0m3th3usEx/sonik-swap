use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct UserId {
    id: Uuid,
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Error)]
pub enum UserIdError {
    // The uuid is not valid
    #[error("NotAValidUuid")]
    NotAValidUuid,
}

impl UserId {
    pub fn new(id: impl TryInto<Uuid>) -> Result<Self, UserIdError> {
        let id = id.try_into().map_err(|_| UserIdError::NotAValidUuid)?;
        if id.is_nil() || id.is_max() {
            return Err(UserIdError::NotAValidUuid);
        }
        Ok(Self { id })
    }

    pub fn value(&self) -> Uuid {
        self.id
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.id
    }
}
