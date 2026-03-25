use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AccionError {
    #[error("Accion no valida: {0}")]
    NoValida(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Accion {
    Leer,
    Escribir,
    Actualizar,
    Eliminar,
}

impl fmt::Display for Accion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Accion::Leer => write!(f, "leer"),
            Accion::Escribir => write!(f, "escribir"),
            Accion::Actualizar => write!(f, "actualizar"),
            Accion::Eliminar => write!(f, "eliminar"),
        }
    }
}

impl FromStr for Accion {
    type Err = AccionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "leer" => Ok(Accion::Leer),
            "escribir" => Ok(Accion::Escribir),
            "actualizar" => Ok(Accion::Actualizar),
            "eliminar" => Ok(Accion::Eliminar),
            _ => Err(AccionError::NoValida(s.to_string())),
        }
    }
}

impl Accion {
    pub fn desde_metodo_http(metodo: &str) -> Result<Self, AccionError> {
        match metodo.to_uppercase().as_str() {
            "GET" => Ok(Accion::Leer),
            "POST" => Ok(Accion::Escribir),
            "PUT" | "PATCH" => Ok(Accion::Actualizar),
            "DELETE" => Ok(Accion::Eliminar),
            _ => Err(AccionError::NoValida(metodo.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accion_from_str_valido() {
        assert_eq!("leer".parse::<Accion>().unwrap(), Accion::Leer);
        assert_eq!("escribir".parse::<Accion>().unwrap(), Accion::Escribir);
        assert_eq!("actualizar".parse::<Accion>().unwrap(), Accion::Actualizar);
        assert_eq!("eliminar".parse::<Accion>().unwrap(), Accion::Eliminar);
    }

    #[test]
    fn test_accion_desde_metodo_http() {
        assert_eq!(Accion::desde_metodo_http("GET").unwrap(), Accion::Leer);
        assert_eq!(Accion::desde_metodo_http("POST").unwrap(), Accion::Escribir);
        assert_eq!(
            Accion::desde_metodo_http("PUT").unwrap(),
            Accion::Actualizar
        );
        assert_eq!(
            Accion::desde_metodo_http("PATCH").unwrap(),
            Accion::Actualizar
        );
        assert_eq!(
            Accion::desde_metodo_http("DELETE").unwrap(),
            Accion::Eliminar
        );
    }

    #[test]
    fn test_accion_desde_metodo_http_invalido() {
        assert!(Accion::desde_metodo_http("OPTIONS").is_err());
    }

    #[test]
    fn test_accion_display() {
        assert_eq!(Accion::Leer.to_string(), "leer");
        assert_eq!(Accion::Escribir.to_string(), "escribir");
        assert_eq!(Accion::Actualizar.to_string(), "actualizar");
        assert_eq!(Accion::Eliminar.to_string(), "eliminar");
    }
}
