use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExamenError {
    #[error("ID del examen inválido")]
    IdInvalido,

    #[error("Título del examen inválido")]
    TituloInvalido,

    #[error("Descripción del examen inválida")]
    DescripcionInvalida,

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
}
