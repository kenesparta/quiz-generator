use quizz_common::domain::value_objects::estado::EstadoGeneralError;
use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluacionError {
    #[error("ID del examen no válido")]
    EvaluacionIdInvalido(#[from] IdError),

    #[error("Error del estado de la evaluacion")]
    EvaluacionEstadoGeneralError(#[from] EstadoGeneralError),

    #[error("El nombre no es valido")]
    NombreNoValido,

    #[error("La descripcion no es valida")]
    DescripcionNoValida,

    #[error("Error al manipular la base de datos: {0:?}")]
    EvaluacionRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Persistencia no finalizada")]
    PersistenciaNoFinalizada,

    #[error("Lectura no finalizada")]
    LecturaNoFinalizada,

    #[error("La evaluacion no existe")]
    EvaluacionNoExiste,
}
