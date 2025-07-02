use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::etiqueta::EtiquetaError;
use crate::pregunta::domain::error::tipo_pregunta::TipoPreguntaError;
use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreguntaError {
    #[error("Respuesta no existe")]
    RespuestaNoExiste,

    #[error("Respuesta incorrecta")]
    RespuestaIncorrecta,

    #[error("AlternativaUnica no existen")]
    AlternativasNoExisten,

    #[error("AlternativaUnica vacias")]
    AlternativasVacias,

    #[error("alternativa o puntaje no ajustado correctamente")]
    AlternativaNoAjustada,

    #[error("Puntos no existen")]
    PuntajeNoExiste,

    #[error("Puntos no existen")]
    DebeTenerUnaSolaRespuesta,

    #[error("puntaje vacio")]
    PuntajeVacio,

    #[error("puntaje no ajustado correctamente")]
    PuntajeNoAjustado,

    #[error("puntaje no coincide con alternativa")]
    PuntajeNoCoincideConAlternativa,

    #[error("Validacion de examenID fallida")]
    PreguntaErrorExamenID(#[from] IdError),

    #[error("Error en la alternativa")]
    PreguntaAlternativaError(#[from] AlternativaError),

    #[error("Error en la alternativa")]
    PreguntaEtiquetaError(#[from] EtiquetaError),

    #[error("Error en el tipo de pregunta")]
    PreguntaTipoPreguntaError(#[from] TipoPreguntaError),

    #[error("Error en el repositorio")]
    PreguntaRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Persistencia no finalizada")]
    PersistenciaNoFinalizada,

    #[error("Lectura no finalizada")]
    LecturaNoFinalizada,

    #[error("Persistencia no finalizada")]
    ActualizacionNoFinalizada,
}
