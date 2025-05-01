use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fecha {
    value: NaiveDate,
}

#[derive(Error, Debug)]
pub enum FechaError {
    #[error("Invalid date format")]
    InvalidFormat,
    #[error("Date out of range")]
    OutOfRange,
}

impl Fecha {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, FechaError> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(|date| Self { value: date })
            .ok_or(FechaError::OutOfRange)
    }

    pub fn from_str(date_str: &str) -> Result<Self, FechaError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map(|date| Self { value: date })
            .map_err(|_| FechaError::InvalidFormat)
    }

    pub fn value(&self) -> NaiveDate {
        self.value
    }
}

impl Display for Fecha {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format("%Y-%m-%d"))
    }
}
