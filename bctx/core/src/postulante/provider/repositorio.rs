use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::id::PostulanteID;

pub trait RepositorioPostulanteEscritura: Send + Sync {
    fn registrar_postulante(
        &self,
        postulante: Postulante,
    ) -> impl Future<Output = Result<(), PostulanteError>> + Send;

    fn actualizar_postulante(
        &self,
        postulante: Postulante,
    ) -> impl Future<Output = Result<(), PostulanteError>> + Send;

    fn eliminar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> impl Future<Output = Result<(), PostulanteError>> + Send;
}

pub trait RepositorioPostulanteLectura: Send + Sync {
    fn obtener_postulante_por_documento(
        &self,
        documento: Documento,
    ) -> impl Future<Output = Result<Postulante, PostulanteError>> + Send;

    fn obtener_postulante_por_id(
        &self,
        postulante_id: PostulanteID,
    ) -> impl Future<Output = Result<Postulante, PostulanteError>> + Send;

    fn obtener_lista_de_postulantes(
        &self,
    ) -> impl Future<Output = Result<Vec<Postulante>, PostulanteError>> + Send;
}
