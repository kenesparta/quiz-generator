use crate::pregunta::domain::error::etiqueta::EtiquetaError;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Etiqueta {
    No,
    Extrovertido,
    Introvertido,
}

impl fmt::Display for Etiqueta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Extrovertido => write!(f, "extrovertido"),
            Self::Introvertido => write!(f, "introvertido"),
        }
    }
}

impl FromStr for Etiqueta {
    type Err = EtiquetaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "no" => Ok(Etiqueta::No),
            "extrovertido" => Ok(Etiqueta::Extrovertido),
            "introvertid" => Ok(Etiqueta::Introvertido),
            _ => Err(EtiquetaError::NoValido),
        }
    }
}
