use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use quizz_core::postulante::domain::error::password::PasswordError;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::provider::password::SeguridadPassword;

pub struct CifradoPorDefecto;

#[async_trait]
impl SeguridadPassword<PostulanteError> for CifradoPorDefecto {
    async fn cifrar(&self, password: String) -> Result<String, PostulanteError> {
        hash(password, DEFAULT_COST)
            .map_err(|_| PostulanteError::PostulantePasswordError(PasswordError::CifradoNoValido))
    }

    async fn comparar(&self, password: String, hashed: String) -> Result<bool, PostulanteError> {
        verify(password, &hashed)
            .map_err(|_| PostulanteError::PostulantePasswordError(PasswordError::NoVerificado))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_cifrar_password_success() {
        let cifrado = CifradoPorDefecto;
        let password = "password123".to_string();

        let result = cifrado.cifrar(password.clone()).await;

        assert!(result.is_ok(), "Password encryption should succeed");
        let hashed = result.unwrap();

        assert_ne!(
            password, hashed,
            "Hashed password should differ from original"
        );
        assert!(hashed.starts_with("$2"), "Hash should be in bcrypt format");
    }

    #[test]
    async fn test_comparar_password_success() {
        let cifrado = CifradoPorDefecto;
        let password = "password123".to_string();

        let hashed = cifrado.cifrar(password.clone()).await.unwrap();
        let result = cifrado.comparar(password, hashed).await;

        assert!(result.is_ok(), "Password comparison should succeed");
        assert!(result.unwrap(), "Passwords should match");
    }

    #[test]
    async fn test_comparar_password_mismatch() {
        let cifrado = CifradoPorDefecto;
        let password = "password123".to_string();
        let wrong_password = "wrong_password".to_string();

        let hashed = cifrado.cifrar(password).await.unwrap();
        let result = cifrado.comparar(wrong_password, hashed).await;

        assert!(
            result.is_ok(),
            "Password comparison should succeed even with mismatched passwords"
        );
        assert!(!result.unwrap(), "Passwords should not match");
    }

    #[test]
    async fn test_comparar_invalid_hash() {
        let cifrado = CifradoPorDefecto;
        let password = "password123".to_string();
        let invalid_hash = "not_a_valid_hash".to_string();

        let result = cifrado.comparar(password, invalid_hash).await;

        assert!(result.is_err(), "Should fail with invalid hash");
        match result {
            Err(PostulanteError::PostulantePasswordError(PasswordError::NoVerificado)) => {}
            _ => panic!("Unexpected error type returned"),
        }
    }
}
