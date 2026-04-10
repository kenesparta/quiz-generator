use crate::admin::domain::admin::AdminLogin;
use async_trait::async_trait;
use quizz_common::domain::entity::jwt::JwtObject;

#[async_trait]
pub trait RepositorioAdminLoginLectura<Error>: Send + Sync {
    async fn obtener_admin_por_documento(&self, documento: String) -> Result<AdminLogin, Error>;
}

#[async_trait]
pub trait RepositorioAdminCacheEscritura<Error>: Send + Sync {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), Error>;
}
