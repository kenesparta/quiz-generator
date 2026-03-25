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
    Examen,
    Evaluacion,
    Postulante,
    Respuesta,
    Revision,
    Usuario,
}

impl fmt::Display for Recurso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Recurso::Examen => write!(f, "examen"),
            Recurso::Evaluacion => write!(f, "evaluacion"),
            Recurso::Postulante => write!(f, "postulante"),
            Recurso::Respuesta => write!(f, "respuesta"),
            Recurso::Revision => write!(f, "revision"),
            Recurso::Usuario => write!(f, "usuario"),
        }
    }
}

impl FromStr for Recurso {
    type Err = RecursoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "examen" => Ok(Recurso::Examen),
            "evaluacion" => Ok(Recurso::Evaluacion),
            "postulante" => Ok(Recurso::Postulante),
            "respuesta" => Ok(Recurso::Respuesta),
            "revision" => Ok(Recurso::Revision),
            "usuario" => Ok(Recurso::Usuario),
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
    fn test_recurso_from_str_valido() {
        assert_eq!("examen".parse::<Recurso>().unwrap(), Recurso::Examen);
        assert_eq!(
            "evaluacion".parse::<Recurso>().unwrap(),
            Recurso::Evaluacion
        );
        assert_eq!(
            "postulante".parse::<Recurso>().unwrap(),
            Recurso::Postulante
        );
        assert_eq!("respuesta".parse::<Recurso>().unwrap(), Recurso::Respuesta);
        assert_eq!("revision".parse::<Recurso>().unwrap(), Recurso::Revision);
    }

    #[test]
    fn test_recurso_desde_ruta() {
        assert_eq!(Recurso::desde_ruta("/examen/123").unwrap(), Recurso::Examen);
        assert_eq!(
            Recurso::desde_ruta("/respuesta/456/postulante/789").unwrap(),
            Recurso::Respuesta
        );
    }

    #[test]
    fn test_recurso_from_str_invalido() {
        assert!("desconocido".parse::<Recurso>().is_err());
    }
}
