use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::pregunta::Puntaje;
use crate::respuesta::domain::entity::respuesta::{Respuesta, RespuestaEvaluacion};
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioRespuestaEscritura<Error>: Send + Sync {
    async fn asignar_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> Result<(), Error>;

    async fn responder_evaluacion(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<(), Error>;

    // Se usa para obtener el puntaje correcto de una pregunta especifica para poder realizar la correccion
    async fn obtener_puntaje(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<Puntaje, Error>;
}

#[async_trait]
pub trait RepositorioRespuestaLectura<Error>: Send + Sync {
    async fn obtener_por_postulante(&self, postulante_id: PostulanteID)
    -> Result<Respuesta, Error>;
}

#[async_trait]
pub trait RespositorioFinalizarEvaluacion<Error>: Send + Sync {
    async fn finalizar_evaluacion(&self, evaluacion_id: EvaluacionID) -> Result<(), Error>;
}
