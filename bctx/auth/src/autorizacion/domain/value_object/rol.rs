use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RolError {
    #[error("Rol no valido: {0}")]
    NoValido(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rol {
    Postulante,
    Psicologo,
    Admin,
}

impl fmt::Display for Rol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rol::Postulante => write!(f, "postulante"),
            Rol::Psicologo => write!(f, "psicologo"),
            Rol::Admin => write!(f, "admin"),
        }
    }
}

impl FromStr for Rol {
    type Err = RolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postulante" => Ok(Rol::Postulante),
            "psicologo" => Ok(Rol::Psicologo),
            "admin" => Ok(Rol::Admin),
            _ => Err(RolError::NoValido(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rol_from_str_valido() {
        assert_eq!("postulante".parse::<Rol>().unwrap(), Rol::Postulante);
        assert_eq!("psicologo".parse::<Rol>().unwrap(), Rol::Psicologo);
        assert_eq!("admin".parse::<Rol>().unwrap(), Rol::Admin);
        assert_eq!("POSTULANTE".parse::<Rol>().unwrap(), Rol::Postulante);
    }

    #[test]
    fn test_rol_from_str_invalido() {
        assert!("desconocido".parse::<Rol>().is_err());
    }

    #[test]
    fn test_rol_display() {
        assert_eq!(Rol::Postulante.to_string(), "postulante");
        assert_eq!(Rol::Psicologo.to_string(), "psicologo");
        assert_eq!(Rol::Admin.to_string(), "admin");
    }
}
