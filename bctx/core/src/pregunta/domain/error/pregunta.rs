use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::etiqueta::EtiquetaError;
use crate::pregunta::domain::error::tipo_pregunta::TipoPreguntaError;
use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreguntaError {
    #[error("La pregunta debe tener alternativas")]
    AlternativasNoExisten,

    #[error("Una pregunta de si o no debe tener las alternativas SI y NO")]
    AlternativaNoAjustada,

    #[error("La pregunta debe tener puntaje")]
    PuntajeNoExiste,

    #[error("La pregunta debe tener una sola respuesta correcta")]
    DebeTenerUnaSolaRespuesta,

    #[error("El puntaje solo puede usar claves de las alternativas")]
    PuntajeNoCoincideConAlternativa,

    #[error("La imagen debe ser un data URI PNG, JPEG o WebP de hasta 512 KiB")]
    ImagenNoValida,

    #[error("ID del examen no valido")]
    PreguntaErrorExamenID(#[from] IdError),

    #[error("Alternativa no valida")]
    PreguntaAlternativaError(#[from] AlternativaError),

    #[error("Etiqueta no valida")]
    PreguntaEtiquetaError(#[from] EtiquetaError),

    #[error("Tipo de pregunta no valido")]
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

    #[error("Actualizacion no finalizada")]
    ActualizacionNoFinalizada,
}
