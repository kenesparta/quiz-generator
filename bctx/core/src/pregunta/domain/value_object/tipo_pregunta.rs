use crate::pregunta::domain::error::tipo_pregunta::TipoPreguntaError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TipoPregunta {
    Alternativas,
    Libre,
    SolaRespuesta,
    SioNo,
}

impl fmt::Display for TipoPregunta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoPregunta::Alternativas => write!(f, "alternativas"),
            TipoPregunta::Libre => write!(f, "libre"),
            TipoPregunta::SolaRespuesta => write!(f, "sola_respuesta"),
            TipoPregunta::SioNo => write!(f, "si_no"),
        }
    }
}

impl FromStr for TipoPregunta {
    type Err = TipoPreguntaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "alternativas" => Ok(TipoPregunta::Alternativas),
            "libre" => Ok(TipoPregunta::Libre),
            "sola_respuesta" => Ok(TipoPregunta::SolaRespuesta),
            "si_no" => Ok(TipoPregunta::SioNo),
            _ => Err(TipoPreguntaError::NoValido),
        }
    }
}
