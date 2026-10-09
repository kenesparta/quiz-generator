use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub documento: String,
}

/// La vista de un postulante es la misma en la búsqueda y en el listado.
pub use crate::postulante::use_case::lista_postulantes::OutputData;

pub struct ObtenerPostulantePorDNI<RepoErr> {
    repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
}

impl<RepoErr> ObtenerPostulantePorDNI<RepoErr> {
    pub fn new(
        repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
    ) -> ObtenerPostulantePorDNI<RepoErr> {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, PostulanteError> for ObtenerPostulantePorDNI<RepoErr>
where
    PostulanteError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let documento = Documento::new(&in_.documento)?;
        let postulante = self
            .repositorio
            .obtener_postulante_por_documento(documento)
            .await?;

        Ok(postulante.into())
    }
}
