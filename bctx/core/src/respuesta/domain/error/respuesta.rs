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

    #[error("La pregunta no existe")]
    PreguntaNotFound,

    #[error("El examen no existe")]
    ExamenNotFound,

    #[error("La evaluacion no esta en proceso")]
    EvaluacionNoEstaEnProceso,
}

#[derive(Error, Debug)]
pub enum EstadoErr {
    #[error("No es un estado valido")]
    NoValido,
}

#[derive(Error, Debug)]
pub enum RevisionErr {
    #[error("No es un estado valido")]
    NoValido,
}
