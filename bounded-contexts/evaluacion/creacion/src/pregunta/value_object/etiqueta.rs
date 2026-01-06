use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EtiquetaError {
    #[error("Etiqueta no valida")]
    NoValido,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Etiqueta {
    No,
    Extrovertido,
    Neurotismo,
    Honestidad,
}

impl fmt::Display for Etiqueta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Extrovertido => write!(f, "extrovertido"),
            Self::Neurotismo => write!(f, "neurotismo"),
            Self::Honestidad => write!(f, "honestidad"),
        }
    }
}

impl FromStr for Etiqueta {
    type Err = EtiquetaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "no" => Ok(Etiqueta::No),
            "extrovertido" => Ok(Etiqueta::Extrovertido),
            "neurotismo" => Ok(Etiqueta::Neurotismo),
            "honestidad" => Ok(Etiqueta::Honestidad),
            _ => Err(EtiquetaError::NoValido),
        }
    }
}
