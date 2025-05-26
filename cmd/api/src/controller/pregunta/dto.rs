use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreguntaInputDto {
    pub preguntas: Vec<PreguntaRawDataDto>,
}

#[derive(Debug, Serialize, Deserialize)]
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
