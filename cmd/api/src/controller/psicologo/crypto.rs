use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use quizz_core::psicologo::domain::error::psicologo::PsicologoError;
use quizz_core::psicologo::provider::password::SeguridadPasswordPsicologo;

pub struct CifradoPsicologo;

#[async_trait]
impl SeguridadPasswordPsicologo<PsicologoError> for CifradoPsicologo {
    async fn cifrar(&self, password: String) -> Result<String, PsicologoError> {
        hash(password, DEFAULT_COST).map_err(|_| PsicologoError::PasswordVacio)
    }
}
