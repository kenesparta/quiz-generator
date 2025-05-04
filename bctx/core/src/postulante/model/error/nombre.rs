use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NombreError {
    #[error("El o los nombres estan vacios")]
    NombreVacio,

    #[error("El apellido esta vacio")]
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
