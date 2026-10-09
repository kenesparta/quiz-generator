use crate::domain::value_objects::zona_horaria::ahora_lima;
use chrono::{NaiveDate, ParseError};
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// La regla es "mayor a 10 años": se exigen 11 años cumplidos.
const EDAD_MINIMA: u32 = 10;
const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Error, Debug)]
pub enum FechaNacimientoError {
    #[error("Formato de fecha no válido")]
    FormatoNoValido(#[from] ParseError),

    #[error("La edad debe ser mayor a 10 años")]
    EdadMinima,
}

#[derive(Debug)]
pub struct FechaNacimiento {
    value: NaiveDate,
}

impl Display for FechaNacimiento {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format(DATE_FORMAT))
    }
}

impl FechaNacimiento {
    /// Fecha de nacimiento de alguien que ya cumplió más de [`EDAD_MINIMA`] años hoy (hora de
    /// Lima).
    pub fn new(fecha: &str) -> Result<Self, FechaNacimientoError> {
        Self::new_al(fecha, ahora_lima().date_naive())
    }

    /// Como [`FechaNacimiento::new`], con la fecha de referencia explícita (los tests no
    /// dependen del reloj).
    pub fn new_al(fecha: &str, hoy: NaiveDate) -> Result<Self, FechaNacimientoError> {
        let value = NaiveDate::parse_from_str(fecha, DATE_FORMAT)?;
        let edad_suficiente = hoy
            .years_since(value)
            .is_some_and(|edad| edad > EDAD_MINIMA);
        if !edad_suficiente {
            return Err(FechaNacimientoError::EdadMinima);
        }
        Ok(FechaNacimiento { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hoy() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 2, 15).unwrap()
    }

    #[test]
    fn test_fecha_valida() {
        assert!(FechaNacimiento::new_al("2000-02-15", hoy()).is_ok());
    }

    #[test]
    fn test_fecha_invalida() {
        let fecha = FechaNacimiento::new_al("fecha-invalida", hoy());
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::FormatoNoValido(_)
        ));
    }

    #[test]
    fn con_exactamente_10_anios_todavia_no_alcanza() {
        // Cumplió 10 años ayer y la regla es "mayor a 10".
        let fecha = FechaNacimiento::new_al("2015-02-16", hoy());
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::EdadMinima
        ));
    }

    #[test]
    fn con_11_anios_cumplidos_alcanza() {
        assert!(FechaNacimiento::new_al("2015-02-15", hoy()).is_ok());
    }

    #[test]
    fn una_fecha_futura_no_alcanza() {
        let fecha = FechaNacimiento::new_al("2030-01-01", hoy());
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::EdadMinima
        ));
    }

    #[test]
    fn new_usa_la_fecha_de_hoy() {
        assert!(FechaNacimiento::new("1990-01-01").is_ok());
    }
}
