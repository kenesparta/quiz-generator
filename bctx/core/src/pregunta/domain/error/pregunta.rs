use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreguntaError {
    #[error("Respuesta no existe")]
    RespuestaNoExiste,

    #[error("Respuesta incorrecta")]
    RespuestaIncorrecta,
}
