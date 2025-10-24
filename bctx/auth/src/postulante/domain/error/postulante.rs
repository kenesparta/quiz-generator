use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostulanteLoginError {
    #[error("NoExisteUsuarioOPassword")]
    NoExisteUsuarioOPassword,

    #[error("Password o Usuario Incorrectos")]
    PasswordUsuarioIncorrectos,

    #[error("Error Al Generar JWT")]
    JWTErrorAlGenerar,

    #[error("El password no ha sido verificado")]
    PostulantePasswordErrorNoVerificado,

    #[error("Error en el Repositorio")]
    RepositorioError,

    #[error("El postulante no se ha encontrado")]
    PostulanteNoEncontrado,

    #[error("Cifrado no valido")]
    CifradoNoValido,

    #[error("Error en el cache")]
    ErrorGenericoCache,
}
