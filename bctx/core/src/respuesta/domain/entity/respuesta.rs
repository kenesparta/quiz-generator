use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::error::respuesta::{EstadoErr, RevisionErr};
use crate::respuesta::domain::value_object::id::RespuestaID;
use chrono::{DateTime, FixedOffset};
use std::fmt;
use std::str::FromStr;

pub struct Respuesta {
    pub id: RespuestaID,
    pub estado: Estado,
    /// RFC 3339; vacío mientras no empezó.
    pub fecha_tiempo_inicio: String,
    /// RFC 3339; vacío mientras no finalizó.
    pub fecha_tiempo_fin: String,
    pub evaluacion: Evaluacion,
    pub postulante: PostulanteID,
    pub revision: Revision,
    pub resultado: String,
}

/// La contestación de una pregunta por el dueño de la hoja de respuestas.
impl Respuesta {
    /// Segundos dedicados al examen: desde el inicio hasta el fin o, si sigue en proceso, hasta
    /// `ahora`. `None` si no empezó o si las fechas guardadas no se pueden leer.
    pub fn segundos_transcurridos(&self, ahora: DateTime<FixedOffset>) -> Option<i64> {
        let inicio = DateTime::parse_from_rfc3339(&self.fecha_tiempo_inicio).ok()?;
        let fin = match self.fecha_tiempo_fin.as_str() {
            "" => ahora,
            fin => DateTime::parse_from_rfc3339(fin).ok()?,
        };
        Some(fin.signed_duration_since(inicio).num_seconds())
    }
}

pub struct RespuestaEvaluacion {
    pub id: RespuestaID,
    /// Dueño de la hoja, tomado del token: el repositorio solo escribe si la hoja es suya.
    pub postulante_id: PostulanteID,
    pub examen_id: String,
    pub pregunta_id: String,
    pub respuestas: Vec<String>,
    pub puntos: u32,
}

/// Ciclo de vida de una hoja de respuestas: `Creado` -> `EnProceso` -> `Finalizado`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluacion::value_object::id::EvaluacionID;
    use crate::respuesta::domain::entity::evaluacion::Evaluacion;

    fn hoja(inicio: &str, fin: &str) -> Respuesta {
        Respuesta {
            id: RespuestaID::new_v4(),
            estado: Estado::EnProceso,
            fecha_tiempo_inicio: inicio.to_string(),
            fecha_tiempo_fin: fin.to_string(),
            evaluacion: Evaluacion {
                id: EvaluacionID::new("2cf52b7a-0ee3-43a9-9b89-4a8baaa22250").unwrap(),
                nombre: String::new(),
                descripcion: String::new(),
                examenes: Vec::new(),
            },
            postulante: PostulanteID::new("e17439e0-79e1-47e3-b5f9-5b54367fa290").unwrap(),
            revision: Revision::SinIniciar,
            resultado: String::new(),
        }
    }

    fn fecha(s: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(s).unwrap()
    }

    #[test]
    fn un_examen_finalizado_mide_hasta_el_fin_y_no_hasta_ahora() {
        let finalizado = hoja("2026-10-05T10:00:00-05:00", "2026-10-05T10:45:00-05:00");
        let dos_dias_despues = fecha("2026-10-07T10:00:00-05:00");
        assert_eq!(
            finalizado.segundos_transcurridos(dos_dias_despues),
            Some(2700)
        );
    }

    #[test]
    fn un_examen_en_proceso_mide_hasta_ahora() {
        let en_proceso = hoja("2026-10-05T10:00:00-05:00", "");
        let ahora = fecha("2026-10-05T10:10:00-05:00");
        assert_eq!(en_proceso.segundos_transcurridos(ahora), Some(600));
    }

    #[test]
    fn un_examen_sin_empezar_no_tiene_tiempo() {
        let ahora = fecha("2026-10-05T10:10:00-05:00");
        assert_eq!(hoja("", "").segundos_transcurridos(ahora), None);
    }

    #[test]
    fn mezcla_fechas_guardadas_en_utc_y_en_hora_de_lima() {
        let mixta = hoja("2026-10-05T15:00:00+00:00", "2026-10-05T10:30:00-05:00");
        let ahora = fecha("2026-10-06T00:00:00-05:00");
        assert_eq!(mixta.segundos_transcurridos(ahora), Some(1800));
    }
}
