use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub documento: String,
}

pub struct OutputData {
    pub id: String,
    pub documento: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub nombre_completo: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}

pub struct ObtenerPostulantePorDocumento<RepoErr> {
    repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
}

impl<RepoErr> ObtenerPostulantePorDocumento<RepoErr> {
    pub fn new(
        repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
    ) -> ObtenerPostulantePorDocumento<RepoErr> {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, PostulanteError>
    for ObtenerPostulantePorDocumento<RepoErr>
where
    PostulanteError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let documento = Documento::new(&in_.documento)?;
        let postulante = self
            .repositorio
            .obtener_postulante_por_documento(documento)
            .await?;

        Ok(OutputData {
            id: postulante.id.to_string(),
            documento: postulante.documento.value().to_string(),
            nombre: postulante.nombre_completo.nombre().to_string(),
            primer_apellido: postulante.nombre_completo.primer_apellido().to_string(),
            segundo_apellido: postulante.nombre_completo.segundo_apellido().to_string(),
            nombre_completo: postulante.nombre_completo.nombre_completo().to_string(),
            fecha_nacimiento: postulante.fecha_nacimiento.to_string(),
            grado_instruccion: postulante.grado_instruccion.to_string(),
            genero: postulante.genero.to_string(),
        })
    }
}
