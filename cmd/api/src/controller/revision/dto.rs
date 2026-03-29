use crate::controller::hateoas::Links;
use serde::{Deserialize, Serialize};

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
