use crate::domain::value_objects::zona_horaria::{ahora_lima, formatear_rfc3339};
use chrono::{DateTime, FixedOffset, Utc};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FechaRegistroError {
    #[error("Formato de fecha de registro no válido")]
    FormatoNoValido(String),

    #[error("La fecha de registro no puede ser futura")]
    FechaFutura,
}

#[derive(Debug)]
pub struct FechaRegistro {
    value: DateTime<FixedOffset>,
}

impl Display for FechaRegistro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", formatear_rfc3339(&self.value))
    }
}

impl FechaRegistro {
    pub fn new(fecha: &str) -> Result<Self, FechaRegistroError> {
        let value = DateTime::parse_from_rfc3339(fecha)
            .map_err(|e| FechaRegistroError::FormatoNoValido(e.to_string()))?;

        let fecha_registro = FechaRegistro { value };

        if fecha_registro.es_fecha_futura() {
            return Err(FechaRegistroError::FechaFutura);
        }

        Ok(fecha_registro)
    }

    pub fn ahora() -> Self {
        FechaRegistro {
            value: ahora_lima(),
        }
    }

    pub fn datetime_fixed(&self) -> &DateTime<FixedOffset> {
        &self.value
    }

    fn es_fecha_futura(&self) -> bool {
        self.value > Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha_registro_valida() {
        let fecha = FechaRegistro::new("2024-06-15T10:30:00.000000+00:00");
        assert!(fecha.is_ok());
    }

    #[test]
    fn test_fecha_registro_con_offset_lima() {
        let fecha = FechaRegistro::new("2024-06-15T05:30:00.000000-05:00");
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
        let fecha = FechaRegistro::new("2099-12-31T23:59:59.000000-05:00");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaRegistroError::FechaFutura
        ));
    }

    #[test]
    fn test_fecha_registro_ahora_tiene_offset_lima() {
        let fecha = FechaRegistro::ahora();
        let resultado = fecha.to_string();
        assert!(resultado.contains("-05:00"));
    }

    #[test]
    fn test_fecha_registro_display() {
        let fecha = FechaRegistro::new("2024-06-15T05:30:00.000000-05:00").unwrap();
        assert_eq!(fecha.to_string(), "2024-06-15T05:30:00.000000-05:00");
    }
}
