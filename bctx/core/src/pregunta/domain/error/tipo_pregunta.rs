use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TipoPreguntaError {
    #[error("Tipo de pregunta no valida")]
    NoValido,
}
