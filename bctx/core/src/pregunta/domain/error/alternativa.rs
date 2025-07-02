use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AlternativaError {
    #[error("Alternativa no válida")]
    NoValido,
}
