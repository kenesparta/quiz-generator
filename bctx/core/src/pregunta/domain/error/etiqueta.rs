use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EtiquetaError {
    #[error("Etiqueta no valida")]
    NoValido,
}
