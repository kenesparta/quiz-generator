use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioEmpezarExamen;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use crate::respuesta::domain::entity::respuesta::Estado;

pub struct InputData {
    pub id: String,
}

pub struct EmpezarExamen<RepoErr> {
    repositorio: Box<dyn RepositorioEmpezarExamen<RepoErr>>,
}

impl<RepoErr> EmpezarExamen<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioEmpezarExamen<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), RespuestaError> for EmpezarExamen<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        let estado = self.repositorio.obtener_estado(in_.id.clone()).await?;

        if !matches!(estado, Estado::Creado) {
            return Err(RespuestaError::EvaluacionYaIniciada);
        }

        self.repositorio.empezar_examen(in_.id).await?;
        Ok(())
    }
}
