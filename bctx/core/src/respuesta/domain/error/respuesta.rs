use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RespuestaError {
    #[error("Error al asignar ID a la respuesta")]
    AsignarIDRespuestaError(#[from] IdError),
}
