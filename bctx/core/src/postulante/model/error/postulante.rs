use crate::postulante::model::error::documento::DocumentoError;
use crate::postulante::model::error::nombre::NombreError;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimientoError;
use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostulanteError {
    #[error("Postulante ID no valido: {0}")]
    PostulanteIdError(#[from] IdError),

    #[error("Documento del postulante no valido: {0}")]
    PostulanteDocumentoError(#[from] DocumentoError),

    #[error("Nombre del postulante no valido: {0}")]
    PostulanteNombreError(#[from] NombreError),

    #[error("FechaNacimiento de nacimiento no valido: {0}")]
    PostulanteFechaNacimientoError(#[from] FechaNacimientoError),
}
