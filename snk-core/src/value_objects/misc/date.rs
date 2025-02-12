use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    date: DateTime<Utc>,
}

#[derive(Debug)]
pub enum DateError {
    InvalidDate,
}

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coudn't convert into Date")
    }
}

impl std::error::Error for DateError {}

impl Date {
    pub fn new(date: impl TryInto<DateTime<Utc>>) -> Result<Self, DateError> {
        let date = date.try_into().map_err(|_| DateError::InvalidDate)?;
        Ok(Self { date })
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.date
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date.to_rfc3339())
    }
}
