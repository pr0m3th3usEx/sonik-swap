use crate::value_objects::user::user_id::UserId;

pub trait UserIdProvider {
    fn generate(&self) -> UserId;
}
