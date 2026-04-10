use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdminLoginError {
    #[error("NoExisteUsuarioOPassword")]
    NoExisteUsuarioOPassword,

    #[error("Password o Documento Incorrectos")]
    PasswordDocumentoIncorrectos,

    #[error("Error Al Generar JWT")]
    JWTErrorAlGenerar,

    #[error("El password no ha sido verificado")]
    AdminPasswordErrorNoVerificado,

    #[error("Error en el Repositorio")]
    RepositorioError,

    #[error("El admin no se ha encontrado")]
    AdminNoEncontrado,

    #[error("Cifrado no valido")]
    CifradoNoValido,

    #[error("Error en el cache")]
    ErrorGenericoCache,
}
