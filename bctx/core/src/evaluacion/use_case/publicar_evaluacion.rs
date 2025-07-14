use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::domain::value_object::evaluacion_estado::EvaluacionEstado;
use crate::evaluacion::provider::repositorio::RepositorioPublicarEvaluacion;
use crate::evaluacion::value_object::id::EvaluacionID;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub evaluacion_id: String,
}

pub struct PublicarEvaluacion<RepoErr> {
    repositorio: Box<dyn RepositorioPublicarEvaluacion<RepoErr>>,
}

impl<RepoErr> PublicarEvaluacion<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioPublicarEvaluacion<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), EvaluacionError> for PublicarEvaluacion<RepoErr>
where
    EvaluacionError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        let evaluacion = self
            .repositorio
            .obtener_evaluacion(EvaluacionID::new(in_.evaluacion_id.as_str())?)
            .await?;

        if evaluacion.esta_publicada() {
            return Err(EvaluacionError::EvaluacionYaFuePublicada);
        }

        self.repositorio.publicar_evaluacion(evaluacion).await?;

        Ok(())
    }
}
