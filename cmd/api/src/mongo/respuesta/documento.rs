//! Una hoja de respuestas tal como se escribe al asignarla: con la copia de la evaluación
//! publicada, sus exámenes y sus preguntas.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct EvaluacionMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<ExamenMongoDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct ExamenMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<PreguntaMongoDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct PreguntaMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub contenido: String,
    pub etiqueta: String,
    pub tipo_de_pregunta: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    pub puntaje: HashMap<String, u32>,
    pub respuestas: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct RespuestaMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub evaluacion: EvaluacionMongoDTO,
    pub postulante_id: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub estado: String,
    pub revision: String,
}
