use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    email: String,
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid email format")
    }
}

impl std::error::Error for EmailError {}

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("regex compiles");

        if !re.is_match(&email) {
            return Err(EmailError::InvalidEmail);
        }
        Ok(Self { email })
    }

    pub fn value(&self) -> String {
        self.email.clone()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use rstest::rstest;

    #[rstest]
    #[case("test@dod.n")]
    #[case("testdod.n")]
    #[case("tes/tdod@gmail.ne")]
    fn is_invalid_email(#[case] email_test: &str) {
        let email = Email::new(email_test.to_string());

        assert!(email.is_err());
    }

    #[rstest]
    #[case("dahyun@edamame.tech")]
    #[case("momo@twice.kr")]
    #[case("tezos22@gmail.com")]
    #[case("chae.won@sseraf.io")]
    #[case("chae.won+test@sseraf.com")]
    fn is_valid_email(#[case] email_test: &str) {
        let email = Email::new(email_test.to_string());

        assert!(email.is_ok());
    }

    #[rstest]
    #[case("toto@edamame.tech")]
    fn test_regex_email(#[case] email_test: String) {
        let split = Email::new(email_test);
        assert!(split.is_ok());
    }
}
