use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioRespuestaRevision;

pub struct OutputData {
    pub revision_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado_revision: String,
    pub postulante_id: String,
    pub fecha_tiempo_fin: String,
}

// Lista las revisiones finalizadas
pub struct RespuestaRevision<R> {
    repo: R,
}

impl<R: RepositorioRespuestaRevision> RespuestaRevision<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn ejecutar(&self) -> Result<Vec<OutputData>, RespuestaError> {
        self.repo
            .obtener_respuesta_revision(Estado::Finalizado)
            .await
    }
}
