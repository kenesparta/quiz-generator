use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UsuarioError {
    #[error("El nombre no puede estar vacio")]
    NombreVacio,

    #[error("El email no puede estar vacio")]
    EmailVacio,

    #[error("El email no es valido: {0}")]
    EmailInvalido(String),

    #[error("El password no puede estar vacio")]
    PasswordVacio,

    #[error("Rol no valido: {0}")]
    RolInvalido(String),

    #[error("Error al cifrar el password")]
    ErrorCifrado,

    #[error("Error en el repositorio")]
    RepositorioError,
}
