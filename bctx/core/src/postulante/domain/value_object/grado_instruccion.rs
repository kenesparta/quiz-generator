use crate::postulante::domain::error::grado_instruccion::GradoInstruccionError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum GradoInstruccion {
    Ninguno,
    Primaria,
    Secundaria,
    Superior,
    Posgrado,
}

impl fmt::Display for GradoInstruccion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ninguno => write!(f, "Ninguno"),
            Self::Primaria => write!(f, "Primaria"),
            Self::Secundaria => write!(f, "Secundaria"),
            Self::Superior => write!(f, "Superior"),
            Self::Posgrado => write!(f, "Posgrado"),
        }
    }
}

impl FromStr for GradoInstruccion {
    type Err = GradoInstruccionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ninguno" => Ok(Self::Ninguno),
            "primaria" => Ok(Self::Primaria),
            "secundaria" => Ok(Self::Secundaria),
            "superior" => Ok(Self::Superior),
            "posgrado" => Ok(Self::Posgrado),
            _ => Err(GradoInstruccionError::NoValido),
        }
    }
}
