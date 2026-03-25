use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsicologoLoginError {
    #[error("NoExisteUsuarioOPassword")]
    NoExisteUsuarioOPassword,

    #[error("Password o Email Incorrectos")]
    PasswordEmailIncorrectos,

    #[error("Error Al Generar JWT")]
    JWTErrorAlGenerar,

    #[error("El password no ha sido verificado")]
    PsicologoPasswordErrorNoVerificado,

    #[error("Error en el Repositorio")]
    RepositorioError,

    #[error("El psicologo no se ha encontrado")]
    PsicologoNoEncontrado,

    #[error("Cifrado no valido")]
    CifradoNoValido,

    #[error("Error en el cache")]
    ErrorGenericoCache,
}
