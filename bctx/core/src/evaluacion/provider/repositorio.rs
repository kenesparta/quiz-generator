use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use crate::evaluacion::value_object::examen_id::ExamenIDs;
use crate::evaluacion::value_object::id::EvaluacionID;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioEvaluacionEscritura<Error>: Send + Sync {
    async fn guardar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), Error>;
    async fn agregar_examen(
        &self,
        evaluacion_id: EvaluacionID,
        examen_ids: ExamenIDs,
    ) -> Result<(), Error>;
}
