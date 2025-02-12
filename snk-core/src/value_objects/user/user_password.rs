use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPassword {
  value: String,
}


#[derive(Debug, Error)]
pub enum UserPasswordError {
  #[error("Invalid password")]
  InvalidPassword,
  #[error("Password is not strong")]
  NotStrong,
}

impl std::fmt::Display for UserPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl UserPassword {
  /// For plain text password
  pub fn new<T: TryInto<String>>(value: T) -> Result<Self, UserPasswordError> {
    let Ok(password) = value.try_into() else {
      return Err(UserPasswordError::InvalidPassword);
    };

    // Password verification
    if password.is_empty() {
      return Err(UserPasswordError::InvalidPassword);
    }

    if password.len() < 8 {
      return Err(UserPasswordError::NotStrong);
    }

    Ok(Self { value: password })
  }
    
  pub fn value(&self) -> String {
    self.value.clone()
  }

  // from hash password
  pub fn from_hash<T: ToString>(value: T) -> Self {
    Self { value: value.to_string() }
  }

  /// For base64 encoded password
  pub fn from_base64(value: String) -> Result<Self, UserPasswordError> {
    BASE64_STANDARD.decode(value)
      .map_err(|_| UserPasswordError::InvalidPassword)
      .and_then(|bytes| {
        let password = String::from_utf8(bytes).map_err(|_| UserPasswordError::InvalidPassword)?;
        Self::new(password)
      })
  }

  pub fn to_base64(&self) -> String {
    BASE64_STANDARD.encode(self.value.clone())
  }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.value
    }
}