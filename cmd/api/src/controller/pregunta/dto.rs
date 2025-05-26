use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PreguntaInputDto {
    pub examen_id: String,
    pub preguntas: Vec<PreguntaRawDataDto>,
}

#[derive(Debug, Serialize)]
pub enum PreguntaRawDataDto {
    Alternativas {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
        alternativa_correcta: String,
        alternativas: HashMap<String, String>,
    },
    Libre {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
    },
    SolaRespuesta {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
        respuesta_correcta: String,
    },
}
