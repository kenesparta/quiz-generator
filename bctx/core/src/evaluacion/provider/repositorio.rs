use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::use_case::listar_evaluaciones::OutputData;
use crate::evaluacion::value_object::examen_id::ExamenIDs;
use crate::evaluacion::value_object::id::EvaluacionID;

pub trait RepositorioEvaluacionEscritura: Send + Sync {
    fn guardar_evaluacion(
        &self,
        evaluacion: Evaluacion,
    ) -> impl Future<Output = Result<(), EvaluacionError>> + Send;

    /// Asocia exámenes existentes a una evaluación en borrador, en una sola operación.
    /// Falla con `ExamenNoExiste`, `EvaluacionNoExiste` o `EvaluacionYaFuePublicada`: una
    /// evaluación publicada guarda una copia de sus exámenes y no se modifica.
    fn agregar_examen(
        &self,
        evaluacion_id: EvaluacionID,
        examen_ids: ExamenIDs,
    ) -> impl Future<Output = Result<(), EvaluacionError>> + Send;
}

pub trait RepositorioLeerEvaluacion: Send + Sync {
    fn obtener_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
    ) -> impl Future<Output = Result<Evaluacion, EvaluacionError>> + Send;
}

pub trait RepositorioPublicarEvaluacion: Send + Sync + RepositorioLeerEvaluacion {
    /// Guarda la evaluación ya publicada (con la copia de sus exámenes) solo si en la base
    /// sigue en borrador; si otra petición la publicó antes, falla con
    /// `EvaluacionYaFuePublicada`.
    fn publicar_evaluacion(
        &self,
        evaluacion: Evaluacion,
    ) -> impl Future<Output = Result<(), EvaluacionError>> + Send;
}

pub trait RepositorioEvaluacionListar: Send + Sync {
    fn listar_evaluaciones(
        &self,
    ) -> impl Future<Output = Result<Vec<OutputData>, EvaluacionError>> + Send;
}
