use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;

pub struct InputData {
    pub documento: String,
}

/// La vista de un postulante es la misma en la búsqueda y en el listado.
pub use crate::postulante::use_case::lista_postulantes::OutputData;

pub struct ObtenerPostulantePorDNI<R> {
    repositorio: R,
}

impl<R: RepositorioPostulanteLectura> ObtenerPostulantePorDNI<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let documento = Documento::new(&in_.documento)?;
        let postulante = self
            .repositorio
            .obtener_postulante_por_documento(documento)
            .await?;

        Ok(postulante.into())
    }
}
