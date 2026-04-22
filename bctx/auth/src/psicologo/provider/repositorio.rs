use crate::psicologo::domain::psicologo::PsicologoLogin;
use async_trait::async_trait;
use quizz_common::domain::entity::jwt::JwtObject;

#[async_trait]
pub trait RepositorioPsicologoLoginLectura<Error>: Send + Sync {
    async fn obtener_psicologo_por_documento(
        &self,
        documento: String,
    ) -> Result<PsicologoLogin, Error>;
}

#[async_trait]
pub trait RepositorioPsicologoCacheEscritura<Error>: Send + Sync {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), Error>;
}
