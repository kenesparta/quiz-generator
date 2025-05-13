use crate::postulante::domain::error::password::PasswordError;
use regex;
use regex::Regex;
use std::sync::OnceLock;

static REGEXP: OnceLock<Regex> = OnceLock::new();

pub fn password_hash_regexp() -> &'static Regex {
    REGEXP.get_or_init(|| {
        Regex::new(r"^\$2[aby]?\$\d{2}\$[./A-Za-z0-9]{53}$")
            .expect("expresion regular para password hash no valida")
    })
}

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, PasswordError> {
        let password = Password { value };
        password.asegurar_password_no_vacio()?;
        password.asegurar_password_hash_valido()?;
        Ok(password)
    }

    pub fn value(self) -> String {
        self.value
    }

    pub fn asegurar_password_no_vacio(&self) -> Result<(), PasswordError> {
        if self.value.is_empty() {
            return Err(PasswordError::Vacio);
        }

        Ok(())
    }

    pub fn asegurar_password_hash_valido(&self) -> Result<(), PasswordError> {
        if !password_hash_regexp().is_match(&self.value) {
            return Err(PasswordError::HashNoValido);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_password_success() {
        let password = Password::new(
            "$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy".to_string(),
        );
        assert!(password.is_ok());
    }

    #[test]
    fn test_asegurar_password_no_vacio_empty() {
        let password = Password::new("".to_string());
        assert!(matches!(password.unwrap_err(), PasswordError::Vacio));
    }

    #[test]
    fn test_asegurar_password_hash_valido_valid() {
        let password = Password::new(
            "$2a$12$NBhpKFs4R0J.lj7.nHwrIe5CmBlvZef/pMxU25EqHjq0VgqCpMOfq".to_string(),
        )
        .unwrap();
        assert!(password.asegurar_password_hash_valido().is_ok());
    }

    #[test]
    fn test_asegurar_password_hash_valido_invalid() {
        let password = Password::new("invalid_hash".to_string());
        assert!(matches!(password.unwrap_err(), PasswordError::HashNoValido));
    }
}
