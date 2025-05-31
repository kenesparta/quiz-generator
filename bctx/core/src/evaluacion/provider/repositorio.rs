use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioEvaluacionEscritura<Error>: Send + Sync {
    async fn guardar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), Error>;
}
