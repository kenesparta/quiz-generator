use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioRespuestaEscritura<Error>: Send + Sync {
    async fn asignar_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> Result<(), Error>;
}
