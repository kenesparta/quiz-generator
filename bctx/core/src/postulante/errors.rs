use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PostulanteError {
    #[error("El ID del postulante esta vacío")]
    IdEsVacio,

    #[error("El ID del postulante no es válido")]
    IdNoValido,
}