use crate::pregunta::domain::error::tipo_pregunta::TipoPreguntaError;
use std::fmt;
use std::str::FromStr;

const ALTERNATIVA_UNICA: &str = "alternativa_unica";
const ALTERNATIVA_PESO: &str = "alternativa_peso";
const LIBRE: &str = "libre";
const SOLA_RESPUESTA: &str = "sola_respuesta";
const SI_O_NO: &str = "si_o_no";

#[derive(Debug, Clone)]
pub enum TipoPregunta {
    AlternativaUnica,
    AlternativaConPeso,
    Libre,
    SolaRespuesta,
    SioNo,
}

impl fmt::Display for TipoPregunta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoPregunta::AlternativaUnica => write!(f, "{}", ALTERNATIVA_UNICA),
            TipoPregunta::AlternativaConPeso => write!(f, "{}", ALTERNATIVA_PESO),
            TipoPregunta::Libre => write!(f, "{}", LIBRE),
            TipoPregunta::SolaRespuesta => write!(f, "{}", SOLA_RESPUESTA),
            TipoPregunta::SioNo => write!(f, "{}", SI_O_NO),
        }
    }
}

impl FromStr for TipoPregunta {
    type Err = TipoPreguntaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            ALTERNATIVA_UNICA => Ok(TipoPregunta::AlternativaUnica),
            ALTERNATIVA_PESO => Ok(TipoPregunta::AlternativaConPeso),
            LIBRE => Ok(TipoPregunta::Libre),
            SOLA_RESPUESTA => Ok(TipoPregunta::SolaRespuesta),
            SI_O_NO => Ok(TipoPregunta::SioNo),
            _ => Err(TipoPreguntaError::NoValido),
        }
    }
}
