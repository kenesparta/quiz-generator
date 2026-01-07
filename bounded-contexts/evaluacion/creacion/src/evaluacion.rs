use common::{Entity, Id, SimpleName};

use std::fmt;
use std::str::FromStr;
use thiserror::Error;

use crate::Examen;

#[derive(Error, Debug, PartialEq)]
pub enum EvaluacionEstadoError {
    #[error("Etiqueta no valida")]
    NoValido,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluacionEstado {
    Borrador,
    Publicado,
    Inactivo,
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
            Self::Inactivo => write!(f, "inactivo"),
        }
    }
}

impl FromStr for EvaluacionEstado {
    type Err = EvaluacionEstadoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "borrador" => Ok(EvaluacionEstado::Borrador),
            "publicado" => Ok(EvaluacionEstado::Publicado),
            "inactivo" => Ok(EvaluacionEstado::Inactivo),
            _ => Err(EvaluacionEstadoError::NoValido),
        }
    }
}

pub struct Evaluacion {
    id: Id,
    titulo: SimpleName,
    descripcion: SimpleName,
    estado: EvaluacionEstado,
    examenes: Vec<Examen>,
}

impl Entity for Evaluacion {
    fn id(&self) -> Id {
        self.id
    }
}
