use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::error::respuesta::{EstadoErr, RevisionErr};
use crate::respuesta::domain::value_object::id::RespuestaID;
use std::fmt;
use std::str::FromStr;

pub struct Respuesta {
    pub id: RespuestaID,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion: Evaluacion,
    pub postulante: PostulanteID,
    pub revision: Revision,
    pub resultado: String,
}

pub struct RespuestaEvaluacion {
    pub id: RespuestaID,
    pub postulante_id: String,
    pub evaluacion_id: String,
    pub examen_id: String,
    pub pregunta_id: String,
    pub respuestas: Vec<String>,
    pub puntos: u32,
}

#[derive(Clone, Debug)]
pub enum Estado {
    Creado,
    EnProceso,
    Finalizado,
}

impl fmt::Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creado => write!(f, "creado"),
            Self::EnProceso => write!(f, "en_proceso"),
            Self::Finalizado => write!(f, "finalizado"),
        }
    }
}

impl FromStr for Estado {
    type Err = EstadoErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "creado" => Ok(Estado::Creado),
            "en_proceso" => Ok(Estado::EnProceso),
            "finalizado" => Ok(Estado::Finalizado),
            _ => Err(EstadoErr::NoValido),
        }
    }
}

impl Estado {
    pub fn can_finalize(&self) -> bool {
        matches!(self, Self::EnProceso)
    }
}

#[derive(Clone, Debug)]
pub enum Revision {
    SinIniciar,
    EnProgreso,
    Finalizada,
    Default,
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SinIniciar => write!(f, "sin_iniciar"),
            Self::EnProgreso => write!(f, "en_proceso"),
            Self::Finalizada => write!(f, "finalizada"),
            Self::Default => write!(f, ""),
        }
    }
}

impl FromStr for Revision {
    type Err = RevisionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sin_iniciar" => Ok(Revision::SinIniciar),
            "en_proceso" => Ok(Revision::EnProgreso),
            "finalizada" => Ok(Revision::Finalizada),
            "" => Ok(Revision::Default),
            _ => Err(RevisionErr::NoValido),
        }
    }
}
