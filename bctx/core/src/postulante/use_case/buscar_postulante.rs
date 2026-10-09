use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;

pub struct InputData {
    pub postulante_id: String,
}

/// La vista de un postulante es la misma en la búsqueda y en el listado.
pub use crate::postulante::use_case::lista_postulantes::OutputData;

pub struct ObtenerPostulantePorId<R> {
    repositorio: R,
}

impl<R: RepositorioPostulanteLectura> ObtenerPostulantePorId<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let postulante_id = PostulanteID::new(&in_.postulante_id)?;
        let postulante = self
            .repositorio
            .obtener_postulante_por_id(postulante_id)
            .await?;

        Ok(postulante.into())
    }
}
