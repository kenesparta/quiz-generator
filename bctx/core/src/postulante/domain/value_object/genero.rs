use crate::postulante::domain::error::genero::GeneroError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Genero {
    Masculino,
    Femenino,
    NoBinario,
}

impl fmt::Display for Genero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Masculino => write!(f, "Masculino"),
            Self::Femenino => write!(f, "Femenino"),
            Self::NoBinario => write!(f, "NoBinario"),
        }
    }
}

impl FromStr for Genero {
    type Err = GeneroError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "masculino" => Ok(Self::Masculino),
            "femenino" => Ok(Self::Femenino),
            "nobinario" => Ok(Self::NoBinario),
            _ => Err(GeneroError::NoValido),
        }
    }
}
