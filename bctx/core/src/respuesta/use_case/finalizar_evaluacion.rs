use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RespositorioFinalizarEvaluacion;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub id: String,
}

pub struct FinalizarEvaluacion<RepoErr> {
    repositorio: Box<dyn RespositorioFinalizarEvaluacion<RepoErr>>,
}

impl<RepoErr> FinalizarEvaluacion<RepoErr> {
    pub fn new(repositorio: Box<dyn RespositorioFinalizarEvaluacion<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), RespuestaError> for FinalizarEvaluacion<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        todo!()
    }
}
