use chrono::{NaiveDate, ParseError};
use std::fmt::{Display, Formatter};
use thiserror::Error;

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
    pub fn new(fecha: &str) -> Result<Self, FechaNacimientoError> {
        let value = NaiveDate::parse_from_str(fecha, DATE_FORMAT)?;
        let fecha_nacimiento = FechaNacimiento { value };

        if !fecha_nacimiento.tiene_edad_minima() {
            return Err(FechaNacimientoError::EdadMinima);
        }

        Ok(fecha_nacimiento)
    }

    fn tiene_edad_minima(&self) -> bool {
        let hoy = chrono::Local::now().date_naive();

        match hoy.years_since(self.value) {
            Some(edad) => edad > EDAD_MINIMA,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha_valida() {
        let fecha = FechaNacimiento::new("2000-02-15");
        assert!(fecha.is_ok());
    }

    #[test]
    fn test_fecha_invalida() {
        let fecha = FechaNacimiento::new("fecha-invalida");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::FormatoNoValido(_)
        ));
    }

    #[test]
    fn test_edad_menor_10_anios() {
        let fecha = FechaNacimiento::new("2020-02-15");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::EdadMinima
        ));
    }

    fn set_fixed_time() {
        unsafe {
            std::env::set_var("CHRONO_OVERRIDE", "2024-02-15T00:00:00Z");
        }
    }

    #[test]
    fn test_edad_exactamente_10_anios() {
        set_fixed_time();
        let fecha = FechaNacimiento::new("2020-02-15");
        assert!(matches!(
            fecha.unwrap_err(),
            FechaNacimientoError::EdadMinima
        ));
    }
}
