use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use crate::evaluacion::use_case::listar_evaluaciones::OutputData;
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

#[async_trait]
pub trait RepositorioLeerEvaluacion<Error>: Send + Sync {
    async fn obtener_evaluacion(&self, evaluacion_id: EvaluacionID) -> Result<Evaluacion, Error>;
}

#[async_trait]
pub trait RepositorioPublicarEvaluacion<Error>:
    Send + Sync + RepositorioLeerEvaluacion<Error>
{
    async fn publicar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), Error>;
}

#[async_trait]
pub trait RepositorioEvaluacionListar<Error>: Send + Sync {
    async fn listar_evaluaciones(&self) -> Result<Vec<OutputData>, Error>;
}
