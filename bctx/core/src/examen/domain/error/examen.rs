use quizz_common::domain::value_objects::estado::EstadoGeneralError;
use quizz_common::domain::value_objects::id::IdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExamenError {
    #[error("ID del examen no válido")]
    ExamenIdInvalido(#[from] IdError),

    #[error("Error general del estado del examen")]
    ExamenEstadoGeneralError(#[from] EstadoGeneralError),

    #[error("Título del examen inválido")]
    TituloInvalido,

    #[error("Descripción del examen inválida")]
    DescripcionInvalida,

    #[error("El puntaje o debe ser cero")]
    PuntajeIgualQueCero,

    #[error("Duración del examen inválida")]
    DuracionInvalida,

    #[error("Puntos totales inválidos")]
    PuntosTotalesInvalidos,

    #[error("Categoría inválida")]
    CategoriaInvalida,

    #[error("Nivel de dificultad inválido")]
    NivelDificultadInvalido,

    #[error("Examen sin preguntas")]
    SinPreguntas,

    #[error("Examen no encontrado")]
    NoEncontrado,

    #[error("Error del repositorio: {0}")]
    RepositorioError(String),

    #[error("Error desconocido: {0}")]
    Desconocido(String),

    #[error("Error al manipular la base de datos: {0:?}")]
    ExamenRepositorioError(#[from] RepositorioError),

    #[error("Tipo de examen no valido")]
    TipoExamenNoValido,
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Persistencia no finalizada")]
    PersistenciaNoFinalizada,

    #[error("Lectura no finalizada")]
    LecturaNoFinalizada,
}
