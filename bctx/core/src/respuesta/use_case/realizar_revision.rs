use crate::respuesta::domain::entity::revision::{ExamenRevision, RevisionRealizada};
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioRealizarRevision;
use chrono::{DateTime, FixedOffset};

pub struct InputData {
    pub respuesta_id: String,
    pub evaluacion_id: String,
    pub resultado: String,
    pub examenes: Vec<InputDataExamen>,
    /// Quien califica: el sujeto del token.
    pub revisado_por: String,
    pub ahora: DateTime<FixedOffset>,
}

pub struct InputDataExamen {
    pub examen_id: String,
    pub observacion: String,
}

/// Califica una hoja finalizada. Solo se califica lo que el postulante ya no puede cambiar.
pub struct RealizarRevision<R> {
    repo: R,
}

impl<R: RepositorioRealizarRevision> RealizarRevision<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        let revision = RevisionRealizada {
            respuesta_id: RespuestaID::new(&in_.respuesta_id)?,
            evaluacion_id: in_.evaluacion_id,
            examenes: in_
                .examenes
                .into_iter()
                .map(|ex| ExamenRevision {
                    examen_id: ex.examen_id,
                    observacion: ex.observacion,
                })
                .collect(),
            resultado: in_.resultado,
            revisado_por: in_.revisado_por,
            fecha: in_.ahora,
        };
        self.repo.realizar_revision(&revision).await
    }
}
