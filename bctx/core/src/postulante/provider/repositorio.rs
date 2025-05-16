use crate::postulante::domain::entity::postulante::Postulante;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPostulanteEscritura<E>: Send + Sync {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), E>;
}
