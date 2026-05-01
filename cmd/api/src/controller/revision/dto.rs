use crate::controller::hateoas::Links;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Request DTOs ---

#[derive(Deserialize)]
pub struct CrearRevisionDTO {
    pub evaluacion_id: String,
    pub examenes: Vec<ExamenRevisionInputDTO>,
    pub resultado: String,
}

#[derive(Deserialize)]
pub struct ExamenRevisionInputDTO {
    pub examen_id: String,
    pub observacion: String,
}

// --- Response DTOs ---

#[derive(Serialize)]
pub struct RevisionListItemDTO {
    pub respuesta_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado_revision: String,
    pub postulante_id: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct RevisionCreatedDTO {
    pub respuesta_id: String,
    pub estado_revision: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct RevisionDetalleDTO {
    pub id: String,
    pub postulante_id: String,
    pub resultado: String,
    pub revision: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psicologo: Option<RevisionPsicologoDTO>,
    pub evaluacion: RevisionEvaluacionDTO,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct RevisionPsicologoDTO {
    pub nombre_completo: String,
    pub colegiatura: String,
}

#[derive(Serialize)]
pub struct RevisionEvaluacionDTO {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<RevisionExamenDTO>,
}

#[derive(Serialize)]
pub struct RevisionExamenDTO {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<RevisionPreguntaDTO>,
    pub puntos_obtenidos: i64,
    pub observacion: String,
}

#[derive(Serialize)]
pub struct RevisionPreguntaDTO {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    pub respuestas: Option<Vec<String>>,
    pub puntos: i64,
}
