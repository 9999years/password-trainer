use std::io;

use dialoguer::{theme::ColorfulTheme, Password as DPassword};
use secrecy::{ExposeSecret, SecretString};
use tracing::instrument;

use super::SecureEq;

/// A password, to be checked against user submissions.
#[derive(Debug)]
pub struct Password {
    password: SecretString,
}

impl From<String> for Password {
    fn from(password: String) -> Self {
        Self {
            password: SecretString::new(password),
        }
    }
}

impl Password {
    /// Tests user input against the stored password.
    #[instrument]
    pub fn test(&self, input: &str) -> Matches {
        if self.password.eq_secure(input) {
            Matches::Correct
        } else {
            Matches::Incorrect
        }
    }

    pub fn test_interactive(&self) -> io::Result<Matches> {
        DPassword::with_theme(&ColorfulTheme::default())
            .with_prompt("password")
            .interact()
            .map(|p| self.test(&p))
    }
}

/// Indicates if a user-provided password matched a stored value.
#[derive(Debug, PartialEq)]
pub enum Matches {
    Correct,
    Incorrect,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_simple() {
        let pw: Password = "abc".to_owned().into();
        assert_eq!(pw.test("abc"), Matches::Correct);
        assert_eq!(pw.test("abd"), Matches::Incorrect);
        assert_eq!(pw.test("abcd"), Matches::Incorrect);
        assert_eq!(pw.test("ab"), Matches::Incorrect);
    }
}
