use crate::postulante::domain::error::password::PasswordError;
use regex::Regex;
use std::fmt;
use std::sync::LazyLock;

/// Formato de un hash bcrypt: `$2a$`, `$2b$` o `$2y$`, el coste y 53 caracteres.
static HASH_BCRYPT: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::expect_used, reason = "patrón literal, cubierto por los tests")]
    Regex::new(r"^\$2[aby]?\$\d{2}\$[./A-Za-z0-9]{53}$").expect("patrón de hash bcrypt no válido")
});

pub fn password_hash_regexp() -> &'static Regex {
    &HASH_BCRYPT
}

/// Hash bcrypt de la contraseña del postulante. `Debug` no muestra el hash.
pub struct Password {
    value: String,
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Password(***)")
    }
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
