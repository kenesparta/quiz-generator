use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NombreError {
    #[error("El o los nombres están vacios")]
    NombreVacio,

    #[error("El apellido está vacio")]
    ApellidoVacio,

    #[error("Los nombres no son válidos")]
    NombreNoValido,

    #[error("El apellido no es válido")]
    ApellidoNoValido,

    #[error("El nombre excede el número maximo de caracteres")]
    NombreExcedeCaracteres,

    #[error("El apellido excede el número maximo de caracteres")]
    ApellidoExcedeCaracteres,
}
