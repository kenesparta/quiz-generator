use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use quizz_core::postulante::domain::error::password::PasswordError;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::provider::password::SeguridadPassword;
use std::error::Error;
use std::fmt;

pub struct CifradoPorDefecto;

impl SeguridadPassword<PostulanteError> for CifradoPorDefecto {
    fn cifrar(&self, password: String) -> Result<String, PostulanteError> {
        hash(password, DEFAULT_COST)
            .map_err(|_| PostulanteError::PostulantePasswordError(PasswordError::CifradoNoValido))
    }

    fn comparar(&self, password: String, hashed: String) -> Result<bool, PostulanteError> {
        verify(password, &hashed)
            .map_err(|_| PostulanteError::PostulantePasswordError(PasswordError::NoVerificado))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cifrar_password() {
        let cifrado = CifradoPorDefecto;
        let password = "test123".to_string();

        let result = cifrado.cifrar(password.clone());
        assert!(result.is_ok());

        let hashed = result.unwrap();
        assert_ne!(password, hashed);
    }

    #[test]
    fn test_comparar_password_valido() {
        let cifrado = CifradoPorDefecto;
        let password = "test123".to_string();
        let hashed = cifrado.cifrar(password.clone()).unwrap();

        let result = cifrado.comparar(password, hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_comparar_password_invalido() {
        let cifrado = CifradoPorDefecto;
        let password = "test123".to_string();
        let wrong_password = "wrong123".to_string();
        let hashed = cifrado.cifrar(password).unwrap();

        let result = cifrado.comparar(wrong_password, hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
