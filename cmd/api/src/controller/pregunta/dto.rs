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
    pub alternativas: Option<HashMap<String, String>>,
    pub puntaje: Option<HashMap<String, u32>>,
}
