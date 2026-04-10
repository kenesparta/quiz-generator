use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdminError {
    #[error("Admin ID no valido: {0}")]
    AdminIdError(#[from] IdError),

    #[error("Nombre no valido: {0}")]
    NombreNoValido(String),

    #[error("Documento no valido: {0}")]
    DocumentoNoValido(String),

    #[error("Password vacio")]
    PasswordVacio,

    #[error("Error al manipular la base de datos: {0:?}")]
    AdminRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Error al persistir")]
    PersistenciaNoFinalizada,

    #[error("Error al leer")]
    LecturaNoFinalizada,

    #[error("El password esta vacio antes de ejecutar la persistencia")]
    PasswordVacio,

    #[error("registro no encontrado")]
    RegistroNoEncontrado,
}
