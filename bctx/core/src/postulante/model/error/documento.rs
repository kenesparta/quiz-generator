use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DocumentoError {
    #[error("El documento enviado no es váldo")]
    DocumentoNoValido,
}
