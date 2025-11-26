use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RespositorioRespuestaRevision;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct OutputData {
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub postulante_id: String,
}

pub struct RespuestaRevision<RepoErr> {
    repo: Box<dyn RespositorioRespuestaRevision<RepoErr>>,
}

impl<RepoErr> RespuestaRevision<RepoErr> {
    pub fn new(repo: Box<dyn RespositorioRespuestaRevision<RepoErr>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<(), Vec<OutputData>, RespuestaError> for RespuestaRevision<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, _in: ()) -> Result<Vec<OutputData>, RespuestaError> {
        let respuestas = self
            .repo
            .obtener_respuesta_revision(Estado::Finalizado)
            .await?;

        Ok(respuestas
            .iter()
            .map(|r| OutputData {
                nombre_evaluacion: r.evaluacion.nombre.clone(),
                descripcion_evaluacion: r.evaluacion.descripcion.clone(),
                postulante_id: r.postulante.to_string(),
            })
            .collect::<Vec<OutputData>>())
    }
}
