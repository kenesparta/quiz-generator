use quizz_common::domain::value_objects::estado::EstadoGeneralError;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EvaluacionEstadoError {
    #[error("Etiqueta no valida")]
    NoValido,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluacionEstado {
    Borrador,
    Publicado,
}

impl Default for EvaluacionEstado {
    fn default() -> Self {
        EvaluacionEstado::Borrador
    }
}

impl fmt::Display for EvaluacionEstado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Borrador => write!(f, "borrador"),
            Self::Publicado => write!(f, "publicado"),
        }
    }
}

impl FromStr for EvaluacionEstado {
    type Err = EvaluacionEstadoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "borrador" => Ok(EvaluacionEstado::Borrador),
            "publicado" => Ok(EvaluacionEstado::Publicado),
            _ => Err(EvaluacionEstadoError::NoValido),
        }
    }
}
