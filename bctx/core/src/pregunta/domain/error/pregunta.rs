use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreguntaError {
    #[error("Respuesta no existe")]
    RespuestaNoExiste,

    #[error("Respuesta incorrecta")]
    RespuestaIncorrecta,

    #[error("Validacion de examenID fallida")]
    PreguntaErrorExamenID(#[from] IdError),

    #[error("Error en el repositorio")]
    PreguntaRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Persistencia no finalizada")]
    PersistenciaNoFinalizada,

    #[error("Lectura no finalizada")]
    LecturaNoFinalizada,

    #[error("Persistencia no finalizada")]
    ActualizacionNoFinalizada,
}
