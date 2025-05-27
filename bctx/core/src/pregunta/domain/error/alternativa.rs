use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AlternativaError {
    #[error("Genero no válido")]
    NoValido,
}
