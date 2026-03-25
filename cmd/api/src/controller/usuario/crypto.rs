use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use quizz_auth::usuario::domain::error::usuario::UsuarioError;
use quizz_common::provider::seguridad::SeguridadCifrar;

pub struct CifradoUsuario;

#[async_trait]
impl SeguridadCifrar<UsuarioError> for CifradoUsuario {
    async fn cifrar(&self, password: String) -> Result<String, UsuarioError> {
        hash(password, DEFAULT_COST).map_err(|_| UsuarioError::ErrorCifrado)
    }
}
