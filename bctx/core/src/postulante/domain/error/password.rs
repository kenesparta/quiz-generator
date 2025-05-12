use crate::postulante::domain::error::documento::DocumentoError;
use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("El o los nombres están vacios")]
    PasswordVacio,

    #[error("El o los nombres están vacios")]
    HashingError(#[from] BcryptError),

    #[error("")]
    PasswordDocumentError(#[from] DocumentoError),
}
