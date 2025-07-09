use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioAsignarPostulante;
use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub evaluacion_id: String,
    pub postulante_id: String,
}

pub struct AsignarEvaluacionAPostulante<RepoErr> {
    repositorio: Box<dyn RepositorioAsignarPostulante<RepoErr>>,
}

impl<RepoErr> AsignarEvaluacionAPostulante<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioAsignarPostulante<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), EvaluacionError> for AsignarEvaluacionAPostulante<RepoErr>
where
    EvaluacionError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        self.repositorio
            .asignar_evaluacion_postulante(
                EvaluacionID::new(in_.evaluacion_id.as_str())?,
                PostulanteID::new(in_.postulante_id.as_str())?,
            )
            .await?;
        Ok(())
    }
}
