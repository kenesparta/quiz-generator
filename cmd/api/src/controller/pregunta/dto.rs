use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreguntaInputDto {
    pub preguntas: Vec<PreguntaDetailDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreguntaDetailDto {
    pub id: String,
    pub contenido: String,
    pub etiqueta: String,
    pub tipo_de_pregunta: String,
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    pub puntaje: HashMap<String, u32>,
}
