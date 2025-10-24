use crate::postulante::domain::postulante::PostulanteLogin;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPostulanteLoginLectura<Error>: Send + Sync {
    async fn obtener_postulante_por_documento(
        &self,
        documento: String,
    ) -> Result<PostulanteLogin, Error>;
}

#[async_trait]
pub trait RepositorioPostulanteCacheEscritura<Error>: Send + Sync {
    async fn guardar_token(&self, jwt: String) -> Result<(), Error>;
}
