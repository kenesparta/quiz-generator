use chrono::{NaiveDate, ParseError};
use std::fmt::{Display, Formatter};
use thiserror::Error;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Error, Debug)]
pub enum FechaValueObjectError {
    #[error("Formato de fecha no válido")]
    FormatoNoValido(#[from] ParseError),
}

#[derive(Debug)]
pub struct FechaValueObject {
    value: NaiveDate,
}

impl Display for FechaValueObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format(DATE_FORMAT))
    }
}

impl FechaValueObject {
    pub fn new(fecha: &str) -> Result<Self, FechaValueObjectError> {
        let value = NaiveDate::parse_from_str(fecha, DATE_FORMAT)?;
        Ok(FechaValueObject { value })
    }
}
