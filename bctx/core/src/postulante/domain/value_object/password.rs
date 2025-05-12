use crate::postulante::domain::error::password::PasswordError;
use crate::postulante::domain::value_object::documento::Documento;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn from_document(document: &Documento) -> Result<Password, PasswordError> {
        let last_four = document.get_last_four_characters()?;
        let value = hash(&last_four, DEFAULT_COST)?;
        Ok(Password { value })
    }

    pub fn from_string(password: String) -> Result<Password, PasswordError> {
        let value = hash(password, DEFAULT_COST)?;
        Ok(Password { value })
    }

    pub fn value(self) -> String {
        self.value
    }

    pub fn ensure_password_not_empty(&self, string: &str) -> Result<(), PasswordError> {
        if string.is_empty() {
            return Err(PasswordError::PasswordVacio);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postulante::domain::value_object::documento::Documento;

    #[test]
    fn test_from_document_success() {
        let documento = Documento::new("12345678".to_string()).unwrap();

        let result = Password::from_document(&documento);
        assert!(result.is_ok());

        let password = result.unwrap();
        assert!(!password.value.is_empty());
        assert!(password.value.starts_with("$2"));
    }

    #[test]
    fn test_from_string_success() {
        let password_str = "secure_password";
        let result = Password::from_string(password_str.to_string());
        assert!(result.is_ok());

        let password = result.unwrap();
        assert!(!password.value.is_empty());
        assert!(password.value.starts_with("$2"));
    }

    #[test]
    fn test_ensure_password_not_empty_success() {
        let password = Password {
            value: String::from("hashed_password"),
        };
        let result = password.ensure_password_not_empty("non_empty_password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_ensure_password_not_empty_error() {
        let password = Password {
            value: String::from("hashed_password"),
        };
        let result = password.ensure_password_not_empty("");
        assert!(result.is_err());
        match result {
            Err(PasswordError::PasswordVacio) => (),
            _ => panic!("Expected PasswordVacio error"),
        }
    }

    #[test]
    fn test_value_method() {
        let expected_value = String::from("hashed_password");
        let password = Password {
            value: expected_value.clone(),
        };
        let result = password.value();
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_hashed_passwords_are_different() {
        let password1 = Password::from_string("test_password".to_string()).unwrap();
        let password2 = Password::from_string("test_password".to_string()).unwrap();

        assert_ne!(password1.value, password2.value);
        assert!(password1.value.starts_with("$2"));
        assert!(password2.value.starts_with("$2"));
    }
}
