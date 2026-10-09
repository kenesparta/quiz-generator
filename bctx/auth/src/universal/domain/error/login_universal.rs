use quizz_common::provider::seguridad::CifradoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginUniversalError {
    #[error("No existe usuario con este documento")]
    UsuarioNoEncontrado,

    #[error("Password incorrecto")]
    PasswordIncorrecto,

    #[error("Error al generar JWT")]
    JWTErrorAlGenerar,

    #[error("Error en el repositorio")]
    RepositorioError,

    #[error("Error en el cache")]
    ErrorGenericoCache,

    #[error("Error al verificar el password: {0}")]
    Cifrado(#[from] CifradoError),
}
