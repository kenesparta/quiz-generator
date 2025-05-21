use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::id::PostulanteID;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPostulanteEscritura<E>: Send + Sync {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), E>;
    async fn actualizar_postulante(&self, postulante_id: PostulanteID) -> Result<(), E>;
    async fn eliminar_postulante(&self, postulante_id: PostulanteID) -> Result<(), E>;
}

#[async_trait]
pub trait RepositorioPostulanteLectura<E>: Send + Sync {
    async fn obtener_postulante_por_documento(&self, documento: Documento)
    -> Result<Postulante, E>;

    async fn obtener_postulante_por_id(&self, postulante_id: PostulanteID)
    -> Result<Postulante, E>;

    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, E>;
}
