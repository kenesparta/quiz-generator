use crate::respuesta::domain::entity::respuesta::Revision;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RespositorioRealizarRevision;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use crate::respuesta::domain::entity::revision::ExamenRevision;

pub struct InputData {
    pub respuesta_id: String,
    pub evaluacion_id: String,
    pub examenes: Vec<InputDataExamen>,
}

pub struct InputDataExamen {
    pub examen_id: String,
    pub observacion: String,
}

pub struct RealizarRevision<RepoErr> {
    repo: Box<dyn RespositorioRealizarRevision<RepoErr>>,
}

impl<RepoErr> RealizarRevision<RepoErr> {
    pub fn new(repo: Box<dyn RespositorioRealizarRevision<RepoErr>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), RespuestaError> for RealizarRevision<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        Ok(self
            .repo
            .realizar_revision(
                in_.respuesta_id,
                in_.evaluacion_id,
                in_.examenes.iter().map(|ex| ExamenRevision{
                    examen_id: ex.examen_id.clone(),
                    observacion: ex.observacion.clone(),
                }).collect::<Vec<ExamenRevision>>(),
                Revision::Finalizada,
            )
            .await?)
    }
}
