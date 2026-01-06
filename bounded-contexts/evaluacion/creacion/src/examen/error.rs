use common::{Id, SimpleNameError};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ExamenError {
    #[error("Índice fuera de rango: {indice}, máximo permitido: {maximo}")]
    IndiceFueraDeRango { indice: usize, maximo: usize },

    #[error("Pregunta no encontrada con ID: {0}")]
    PreguntaNoEncontrada(Id),

    #[error("El examen no tiene preguntas")]
    ExamenVacio,

    #[error("Título inválido: {0}")]
    TituloInvalido(#[source] SimpleNameError),

    #[error("Descripción inválida: {0}")]
    DescripcionInvalida(#[source] SimpleNameError),

    #[error("Instrucciones inválidas: {0}")]
    InstruccionesInvalidas(#[source] SimpleNameError),
}
