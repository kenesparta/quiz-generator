use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DocumentoError {
    #[error("El documento enviado no es váldo")]
    DocumentoNoValido,
}

#[derive(Error, Debug, PartialEq)]
pub enum NombreError {
    #[error("Los nombres no son válidos")]
    NombreNoValido,

    #[error("El apellido no es válido")]
    ApellidosNoValidos,
}
