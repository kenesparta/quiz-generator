use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RecursoError {
    #[error("Recurso no valido: {0}")]
    NoValido(String),
}

/// Tipo de recurso protegido. Cada scope de rutas declara el suyo; no se deduce de la URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            "admin" => Ok(Recurso::Admin),
            "examen" => Ok(Recurso::Examen),
            "evaluacion" => Ok(Recurso::Evaluacion),
            "postulante" => Ok(Recurso::Postulante),
            "psicologo" => Ok(Recurso::Psicologo),
            "respuesta" => Ok(Recurso::Respuesta),
            "revision" => Ok(Recurso::Revision),
            _ => Err(RecursoError::NoValido(s.to_string())),
        }
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
    fn test_recurso_display_siempre_singular() {
        assert_eq!(Recurso::Respuesta.to_string(), "respuesta");
        assert_eq!(Recurso::Revision.to_string(), "revision");
        assert_eq!(Recurso::Examen.to_string(), "examen");
    }

    #[test]
    fn test_recurso_from_str_invalido() {
        assert!("desconocido".parse::<Recurso>().is_err());
        // Los nombres de las rutas no son recursos: el recurso se declara en cada scope.
        assert!("admins".parse::<Recurso>().is_err());
    }
}
