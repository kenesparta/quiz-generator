use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AutorizacionError {
    #[error("Acceso denegado")]
    AccesoDenegado,

    #[error("Token no valido")]
    TokenNoValido,

    #[error("Token expirado")]
    TokenExpirado,

    #[error("Error interno del enforzador de politicas")]
    ErrorEnforzador,
}
