use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EstadoGeneralError {
    #[error("Etiqueta no valida")]
    NoValido,
}

#[derive(Clone, Debug)]
pub enum EstadoGeneral {
    Activo,
    Inactivo,
}

impl Default for EstadoGeneral {
    fn default() -> Self {
        Self::Activo
    }
}

impl fmt::Display for EstadoGeneral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Activo => write!(f, "activo"),
            Self::Inactivo => write!(f, "inactivo"),
        }
    }
}

impl FromStr for EstadoGeneral {
    type Err = EstadoGeneralError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoGeneral::Activo),
            "inactivo" => Ok(EstadoGeneral::Inactivo),
            _ => Err(EstadoGeneralError::NoValido),
        }
    }
}
