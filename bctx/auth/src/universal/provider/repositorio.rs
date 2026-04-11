use crate::universal::domain::usuario_login::UsuarioLogin;
use async_trait::async_trait;
use quizz_common::domain::entity::jwt::JwtObject;

#[async_trait]
pub trait RepositorioLoginUniversalLectura<Error>: Send + Sync {
    async fn buscar_por_documento(&self, documento: String) -> Result<UsuarioLogin, Error>;
}

#[async_trait]
pub trait RepositorioLoginUniversalCacheEscritura<Error>: Send + Sync {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), Error>;
}
