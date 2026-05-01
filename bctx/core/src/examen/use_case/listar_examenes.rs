use crate::examen::domain::error::examen::ExamenError;
use crate::examen::provider::repositorio::RepositorioExamenListar;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData;

#[derive(Debug, Clone)]
pub struct OutputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub estado: String,
    pub cantidad_preguntas: usize,
}

pub struct ListarExamenes<RepoErr> {
    repositorio: Box<dyn RepositorioExamenListar<RepoErr>>,
}

impl<RepoErr> ListarExamenes<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioExamenListar<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, Vec<OutputData>, ExamenError> for ListarExamenes<RepoErr>
where
    ExamenError: From<RepoErr>,
{
    async fn ejecutar(&self, _input: InputData) -> Result<Vec<OutputData>, ExamenError> {
        let examenes = self.repositorio.listar_examenes().await?;
        Ok(examenes)
    }
}
