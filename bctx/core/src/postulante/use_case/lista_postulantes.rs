use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::provider::repositorio::RepositorioPostulanteLectura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {}

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

pub struct ListaOutput {
    pub postulantes: Vec<OutputData>,
}

impl From<Postulante> for OutputData {
    fn from(p: Postulante) -> Self {
        Self {
            id: p.id.to_string(),
            documento: p.documento.to_string(),
            nombre: p.nombre_completo.nombre().to_string(),
            primer_apellido: p.nombre_completo.primer_apellido().to_string(),
            segundo_apellido: p.nombre_completo.segundo_apellido().to_string(),
            nombre_completo: p.nombre_completo.nombre_completo().to_string(),
            fecha_nacimiento: p.fecha_nacimiento.to_string(),
            grado_instruccion: p.grado_instruccion.to_string(),
            genero: p.genero.to_string(),
        }
    }
}

pub struct ObtenerListaDePostulantes<RepoErr> {
    repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
}

impl<RepoErr> ObtenerListaDePostulantes<RepoErr> {
    pub fn new(
        repositorio: Box<dyn RepositorioPostulanteLectura<RepoErr>>,
    ) -> ObtenerListaDePostulantes<RepoErr> {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, ListaOutput, PostulanteError>
    for ObtenerListaDePostulantes<RepoErr>
where
    PostulanteError: From<RepoErr>,
{
    async fn ejecutar(&self, _in: InputData) -> Result<ListaOutput, PostulanteError> {
        let lista_de_postulantes = self.repositorio.obtener_lista_de_postulantes().await?;
        Ok(ListaOutput {
            postulantes: lista_de_postulantes.into_iter().map(|p| p.into()).collect(),
        })
    }
}
