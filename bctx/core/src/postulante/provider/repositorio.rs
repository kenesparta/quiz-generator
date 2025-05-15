use crate::postulante::domain::entity::postulante::Postulante;

pub trait RepositorioPostulanteEscritura<E> {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), E>;
}
