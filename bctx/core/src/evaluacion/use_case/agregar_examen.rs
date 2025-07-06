use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioEvaluacionEscritura;
use crate::evaluacion::value_object::examen_id::ExamenIDs;
use crate::evaluacion::value_object::id::EvaluacionID;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub evaluacion_id: String,
    pub examen_ids: Vec<String>,
}

pub struct AgregarExamenAEvaluacion<RepoErr> {
    repositorio: Box<dyn RepositorioEvaluacionEscritura<RepoErr>>,
}

impl<RepoErr> AgregarExamenAEvaluacion<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioEvaluacionEscritura<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), EvaluacionError> for AgregarExamenAEvaluacion<RepoErr>
where
    EvaluacionError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        let evaluacion_id = EvaluacionID::new(in_.evaluacion_id.as_str())?;
        let examen_ids = ExamenIDs::new(in_.examen_ids);
        self.repositorio
            .agregar_examen(evaluacion_id, examen_ids)
            .await?;
        Ok(())
    }
}
