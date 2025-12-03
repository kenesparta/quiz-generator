use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RevisarEvaluacionPostulanteReviewDTO {
    pub respuesta_id: String,
    pub evaluacion_id: String,
    pub examenes: Vec<ExamenRevisionDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct ExamenRevisionDTO {
    pub examen_id: String,
    pub observacion: String,
}
