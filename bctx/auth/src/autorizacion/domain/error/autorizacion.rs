use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AutorizacionError {
    #[error("Acceso denegado")]
    AccesoDenegado,

    #[error("Rol no valido: {0}")]
    RolNoValido(String),

    #[error("Token no valido")]
    TokenNoValido,

    #[error("Token expirado")]
    TokenExpirado,

    #[error("Token no encontrado en la solicitud")]
    TokenNoEncontrado,

    #[error("Error interno del enforzador de politicas")]
    ErrorEnforzador,

    #[error("Recurso no valido: {0}")]
    RecursoNoValido(String),

    #[error("Accion no valida: {0}")]
    AccionNoValida(String),
}
