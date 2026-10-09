use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RespuestaError {
    #[error("ID no válido: {0}")]
    AsignarIDRespuestaError(#[from] IdError),

    #[error("Error al guardar la respuesta en la base de datos")]
    DatabaseError,

    #[error("La evaluacion a ser asignada no existe")]
    EvaluacionRespuestaNotFound,

    #[error("El postulante a ser asignado no existe")]
    PostulanteRespuestaNotFound,

    #[error("La evaluacion ya fue asignada")]
    EvaluacionAlreadyAssigned,

    #[error("La evaluacion no esta publicada")]
    EvaluacionNoPublicada,

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

    #[error("La evaluacion ya fue iniciada")]
    EvaluacionYaIniciada,

    #[error("La evaluacion ya fue finalizada")]
    EvaluacionFinalizada,

    #[error("La evaluacion todavia no fue finalizada")]
    EvaluacionNoFinalizada,

    #[error("El estado de la evaluacion cambio mientras se procesaba la solicitud")]
    TransicionNoAplicada,

    #[error("Solo se admite una respuesta por pregunta")]
    CantidadDeRespuestasNoValida,

    #[error("La respuesta no corresponde a una alternativa de la pregunta o es demasiado larga")]
    RespuestaNoValida,
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
