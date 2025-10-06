use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioRespuestaEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {}

#[derive(Debug, Clone)]
pub struct OutputData {}

pub struct RespuestaEvaluacion<RepoErr> {
    reposorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>,
}

impl<RepoErr> RespuestaEvaluacion<RepoErr> {
    pub fn new(reposorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>) -> Self {
        Self { reposorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, RespuestaError> for RespuestaEvaluacion<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, RespuestaError> {
        Err(RespuestaError::RepositorioError)
    }
}
