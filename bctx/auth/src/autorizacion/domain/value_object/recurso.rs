use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RecursoError {
    #[error("Recurso no valido: {0}")]
    NoValido(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Recurso {
    Admin,
    Examen,
    Evaluacion,
    Postulante,
    Psicologo,
    Respuesta,
    Revision,
}

impl fmt::Display for Recurso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Recurso::Admin => write!(f, "admin"),
            Recurso::Examen => write!(f, "examen"),
            Recurso::Evaluacion => write!(f, "evaluacion"),
            Recurso::Postulante => write!(f, "postulante"),
            Recurso::Psicologo => write!(f, "psicologo"),
            Recurso::Respuesta => write!(f, "respuesta"),
            Recurso::Revision => write!(f, "revision"),
        }
    }
}

impl FromStr for Recurso {
    type Err = RecursoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" | "admins" => Ok(Recurso::Admin),
            "examen" | "examenes" => Ok(Recurso::Examen),
            "evaluacion" | "evaluaciones" => Ok(Recurso::Evaluacion),
            "postulante" | "postulantes" => Ok(Recurso::Postulante),
            "psicologo" | "psicologos" => Ok(Recurso::Psicologo),
            "respuesta" | "respuestas" => Ok(Recurso::Respuesta),
            "revision" | "revisiones" => Ok(Recurso::Revision),
            _ => Err(RecursoError::NoValido(s.to_string())),
        }
    }
}

impl Recurso {
    pub fn desde_ruta(ruta: &str) -> Result<Self, RecursoError> {
        let segmento = ruta.trim_start_matches('/').split('/').next().unwrap_or("");

        segmento.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurso_from_str_singular() {
        assert_eq!("admin".parse::<Recurso>().unwrap(), Recurso::Admin);
        assert_eq!("examen".parse::<Recurso>().unwrap(), Recurso::Examen);
        assert_eq!(
            "evaluacion".parse::<Recurso>().unwrap(),
            Recurso::Evaluacion
        );
        assert_eq!(
            "postulante".parse::<Recurso>().unwrap(),
            Recurso::Postulante
        );
        assert_eq!("psicologo".parse::<Recurso>().unwrap(), Recurso::Psicologo);
        assert_eq!("respuesta".parse::<Recurso>().unwrap(), Recurso::Respuesta);
        assert_eq!("revision".parse::<Recurso>().unwrap(), Recurso::Revision);
    }

    #[test]
    fn test_recurso_from_str_plural() {
        assert_eq!("admins".parse::<Recurso>().unwrap(), Recurso::Admin);
        assert_eq!("examenes".parse::<Recurso>().unwrap(), Recurso::Examen);
        assert_eq!(
            "evaluaciones".parse::<Recurso>().unwrap(),
            Recurso::Evaluacion
        );
        assert_eq!(
            "postulantes".parse::<Recurso>().unwrap(),
            Recurso::Postulante
        );
        assert_eq!("psicologos".parse::<Recurso>().unwrap(), Recurso::Psicologo);
        assert_eq!("respuestas".parse::<Recurso>().unwrap(), Recurso::Respuesta);
        assert_eq!("revisiones".parse::<Recurso>().unwrap(), Recurso::Revision);
    }

    #[test]
    fn test_recurso_desde_ruta_plural() {
        assert_eq!(
            Recurso::desde_ruta("/examenes/123").unwrap(),
            Recurso::Examen
        );
        assert_eq!(
            Recurso::desde_ruta("/respuestas/456").unwrap(),
            Recurso::Respuesta
        );
        assert_eq!(
            Recurso::desde_ruta("/revisiones").unwrap(),
            Recurso::Revision
        );
    }

    #[test]
    fn test_recurso_desde_ruta_singular() {
        assert_eq!(Recurso::desde_ruta("/examen/123").unwrap(), Recurso::Examen);
        assert_eq!(
            Recurso::desde_ruta("/respuesta/456/postulante/789").unwrap(),
            Recurso::Respuesta
        );
    }

    #[test]
    fn test_recurso_display_siempre_singular() {
        assert_eq!(Recurso::Respuesta.to_string(), "respuesta");
        assert_eq!(Recurso::Revision.to_string(), "revision");
        assert_eq!(Recurso::Examen.to_string(), "examen");
    }

    #[test]
    fn test_recurso_from_str_invalido() {
        assert!("desconocido".parse::<Recurso>().is_err());
    }
}
