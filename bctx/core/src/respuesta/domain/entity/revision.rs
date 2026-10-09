use crate::respuesta::domain::value_object::id::RespuestaID;
use chrono::{DateTime, FixedOffset};

pub struct ExamenRevision {
    pub examen_id: String,
    pub observacion: String,
}

/// La calificación de una hoja finalizada, con quién la hizo y cuándo.
pub struct RevisionRealizada {
    pub respuesta_id: RespuestaID,
    pub evaluacion_id: String,
    pub examenes: Vec<ExamenRevision>,
    pub resultado: String,
    /// Id del psicólogo o admin que califica (el sujeto del token).
    pub revisado_por: String,
    pub fecha: DateTime<FixedOffset>,
}
