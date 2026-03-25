use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use quizz_auth::admin::domain::error::admin::AdminLoginError;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::psicologo::domain::error::psicologo::PsicologoLoginError;
use quizz_common::provider::seguridad::{SeguridadCifrar, SeguridadComparar};

pub struct CifradoPorDefecto;

#[async_trait]
impl SeguridadComparar<PostulanteLoginError> for CifradoPorDefecto {
    async fn comparar(&self, password: String, hashed: String) -> Result<(), PostulanteLoginError> {
        match verify(password, &hashed) {
            Ok(true) => Ok(()),
            _ => Err(PostulanteLoginError::PostulantePasswordErrorNoVerificado),
        }
    }
}

#[async_trait]
impl SeguridadCifrar<PostulanteLoginError> for CifradoPorDefecto {
    async fn cifrar(&self, password: String) -> Result<String, PostulanteLoginError> {
        hash(password, DEFAULT_COST).map_err(|_| PostulanteLoginError::CifradoNoValido)
    }
}

#[async_trait]
impl SeguridadComparar<PsicologoLoginError> for CifradoPorDefecto {
    async fn comparar(&self, password: String, hashed: String) -> Result<(), PsicologoLoginError> {
        match verify(password, &hashed) {
            Ok(true) => Ok(()),
            _ => Err(PsicologoLoginError::PsicologoPasswordErrorNoVerificado),
        }
    }
}

#[async_trait]
impl SeguridadComparar<AdminLoginError> for CifradoPorDefecto {
    async fn comparar(&self, password: String, hashed: String) -> Result<(), AdminLoginError> {
        match verify(password, &hashed) {
            Ok(true) => Ok(()),
            _ => Err(AdminLoginError::AdminPasswordErrorNoVerificado),
        }
    }
}
