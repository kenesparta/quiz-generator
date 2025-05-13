use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GeneroError {
    #[error("Genero no válido")]
    NoValido,
}
