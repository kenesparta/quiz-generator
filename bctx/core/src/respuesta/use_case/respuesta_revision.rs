use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioRespuestaRevision;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct OutputData {
    pub revision_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado_revision: String,
    pub postulante_id: String,
    pub fecha_tiempo_fin: String,
}

// Lista las revisiones finalizadas
pub struct RespuestaRevision<RepoErr> {
    repo: Box<dyn RepositorioRespuestaRevision<RepoErr>>,
}

impl<RepoErr> RespuestaRevision<RepoErr> {
    pub fn new(repo: Box<dyn RepositorioRespuestaRevision<RepoErr>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<(), Vec<OutputData>, RespuestaError> for RespuestaRevision<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, _in: ()) -> Result<Vec<OutputData>, RespuestaError> {
        Ok(self
            .repo
            .obtener_respuesta_revision(Estado::Finalizado)
            .await?)
    }
}
