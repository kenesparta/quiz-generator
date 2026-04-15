use chrono::{Local, NaiveDateTime, ParseError};
use std::fmt::{Display, Formatter};
use thiserror::Error;

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Error, Debug)]
pub enum FechaRegistroError {
    #[error("Formato de fecha de registro no válido")]
    FormatoNoValido(#[from] ParseError),

    #[error("La fecha de registro no puede ser futura")]
    FechaFutura,
}

#[derive(Debug)]
pub struct FechaRegistro {
    value: NaiveDateTime,
}

impl Display for FechaRegistro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format(DATETIME_FORMAT))
    }
}

impl FechaRegistro {
    pub fn new(fecha: &str) -> Result<Self, FechaRegistroError> {
        let value = NaiveDateTime::parse_from_str(fecha, DATETIME_FORMAT)?;
        let fecha_registro = FechaRegistro { value };

        if fecha_registro.es_fecha_futura() {
            return Err(FechaRegistroError::FechaFutura);
        }

        Ok(fecha_registro)
    }

    pub fn ahora() -> Self {
        FechaRegistro {
            value: Local::now().naive_local(),
        }
    }

    pub fn naive_datetime(&self) -> &NaiveDateTime {
        &self.value
    }

    fn es_fecha_futura(&self) -> bool {
        self.value > Local::now().naive_local()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha_registro_valida() {
        let fecha = FechaRegistro::new("2024-06-15 10:30:00");
        assert!(fecha.is_ok());
    }

    #[test]
    fn test_fecha_registro_formato_invalido() {
        let fecha = FechaRegistro::new("fecha-invalida");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaRegistroError::FormatoNoValido(_)
        ));
    }

    #[test]
    fn test_fecha_registro_futura() {
        let fecha = FechaRegistro::new("2099-12-31 23:59:59");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaRegistroError::FechaFutura
        ));
    }

    #[test]
    fn test_fecha_registro_ahora() {
        let fecha = FechaRegistro::ahora();
        let resultado = fecha.to_string();
        assert!(!resultado.is_empty());
    }

    #[test]
    fn test_fecha_registro_display() {
        let fecha = FechaRegistro::new("2024-06-15 10:30:00").unwrap();
        assert_eq!(fecha.to_string(), "2024-06-15 10:30:00");
    }
}
