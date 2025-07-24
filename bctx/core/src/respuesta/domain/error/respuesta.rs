use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RespuestaError {
    #[error("Error al asignar ID a la respuesta")]
    AsignarIDRespuestaError(#[from] IdError),

    #[error("Error al guardar la respuesta en la base de datos")]
    DatabaseError,

    #[error("La evaluacion a ser asignada no existe")]
    EvaluacionRespuestaNotFound,

    #[error("El postulante a ser asignado no existe")]
    PostulanteRespuestaNotFound,

    #[error("La evaluacion ya fue asignada")]
    EvaluacionAlreadyAssigned,

    #[error("Error en el repositorio")]
    RepositorioError,

    #[error("La respuesta no existe")]
    RespuestaNoEncontrada,
}
