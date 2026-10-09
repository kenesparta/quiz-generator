use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioRespuestaEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub evaluacion_id: String,
    pub postulante_id: String,
}

/// La hoja de respuestas creada.
pub struct OutputData {
    pub id: RespuestaID,
}

/// Asigna una evaluación publicada a un postulante creando su hoja de respuestas.
pub struct AsignarEvaluacionAPostulante<RepoErr> {
    repositorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>,
}

impl<RepoErr> AsignarEvaluacionAPostulante<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, RespuestaError>
    for AsignarEvaluacionAPostulante<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, RespuestaError> {
        let id = RespuestaID::new_v4();
        self.repositorio
            .asignar_evaluacion(
                &id,
                EvaluacionID::new(in_.evaluacion_id.as_str())?,
                PostulanteID::new(in_.postulante_id.as_str())?,
            )
            .await?;
        Ok(OutputData { id })
    }
}
