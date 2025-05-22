use crate::postulante::domain::error::documento::DocumentoError;
use crate::postulante::domain::error::genero::GeneroError;
use crate::postulante::domain::error::grado_instruccion::GradoInstruccionError;
use crate::postulante::domain::error::nombre::NombreError;
use crate::postulante::domain::error::password::PasswordError;
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

    #[error("Error en el password: {0}")]
    PostulantePasswordError(#[from] PasswordError),

    #[error("Error en el grado de instruccion: {0}")]
    PostulanteGradoInstruccionError(#[from] GradoInstruccionError),

    #[error("Error en el genero: {0}")]
    PostulanteGeneroError(#[from] GeneroError),

    #[error("El password no coincide")]
    PasswordNoCoincide,

    #[error("Error al persistir")]
    PostulanteRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Error al persistir")]
    PersistenciaNoFinalizada,

    #[error("El password esta vacio antes de ejecutar la persistencia")]
    PasswordVacio,

    #[error("registro no encontrado")]
    RegistroNoEncontrado,
}
