use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioEvaluacionListar;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData;

#[derive(Debug, Clone)]
pub struct OutputData {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub estado: String,
    pub esta_activo: String,
    pub cantidad_examenes: usize,
}

pub struct ListarEvaluaciones<RepoErr> {
    repositorio: Box<dyn RepositorioEvaluacionListar<RepoErr>>,
}

impl<RepoErr> ListarEvaluaciones<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioEvaluacionListar<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, Vec<OutputData>, EvaluacionError> for ListarEvaluaciones<RepoErr>
where
    EvaluacionError: From<RepoErr>,
{
    async fn ejecutar(&self, _input: InputData) -> Result<Vec<OutputData>, EvaluacionError> {
        let evaluaciones = self.repositorio.listar_evaluaciones().await?;
        Ok(evaluaciones)
    }
}
