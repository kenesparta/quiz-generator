use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluacionError {
    #[error("ID del examen no válido")]
    EvaluacionIdInvalido(#[from] IdError),

    #[error("El nombre no es valido")]
    NombreNoValido,

    #[error("La descripcion no es valida")]
    DescripcionNoValida,
}
