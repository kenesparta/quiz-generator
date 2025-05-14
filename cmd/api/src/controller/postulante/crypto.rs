use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use quizz_core::postulante::provider::password::SeguridadPassword;

pub struct CifradoPorDefecto {}

impl SeguridadPassword<BcryptError> for CifradoPorDefecto {
    fn cifrar(&self, password: String) -> Result<String, BcryptError> {
        let value = hash(password, DEFAULT_COST)?;
        Ok(value)
    }

    fn comparar(&self, password: String, hashed: String) -> Result<bool, BcryptError> {
        verify(password, &hashed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cifrar_password() {
        let cifrado = CifradoPorDefecto {};
        let password = "test123".to_string();

        let result = cifrado.cifrar(password.clone());
        assert!(result.is_ok());

        let hashed = result.unwrap();
        assert_ne!(password, hashed);
    }

    #[test]
    fn test_comparar_password_valido() {
        let cifrado = CifradoPorDefecto {};
        let password = "test123".to_string();
        let hashed = cifrado.cifrar(password.clone()).unwrap();

        let result = cifrado.comparar(password, hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_comparar_password_invalido() {
        let cifrado = CifradoPorDefecto {};
        let password = "test123".to_string();
        let wrong_password = "wrong123".to_string();
        let hashed = cifrado.cifrar(password).unwrap();

        let result = cifrado.comparar(wrong_password, hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
