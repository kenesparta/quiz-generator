use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RevisarEvaluacionPostulanteReadDTO {
    evaluacion_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RevisarEvaluacionPostulanteReviewDTO {
    pub evaluacion_id: String,
    pub examen_id: String,
    pub observation: String,
}

#[derive(Serialize, Deserialize)]
pub struct RevisarEvaluacionPostulanteFinalizeDTO {
    pub estado: String,
}
