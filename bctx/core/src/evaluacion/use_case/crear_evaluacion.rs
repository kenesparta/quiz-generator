use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioEvaluacionEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
}

pub struct CrearEvaluacion<RepoErr> {
    repositorio: Box<dyn RepositorioEvaluacionEscritura<RepoErr>>,
}

impl<RepoErr> CrearEvaluacion<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioEvaluacionEscritura<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), EvaluacionError> for CrearEvaluacion<RepoErr>
where
    EvaluacionError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        self.repositorio
            .guardar_evaluacion(Evaluacion::new(in_.id, in_.titulo, in_.descripcion)?)
            .await?;
        Ok(())
    }
}
