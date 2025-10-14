use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::respuesta::domain::entity::respuesta::RespuestaEvaluacion;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioRespuestaEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    pub postulante_id: String,
    pub evaluacion_id: String,
    pub examen_id: String,
    pub pregunta_id: String,
    pub respuestas: Vec<String>,
}

pub struct ResponderEvaluacion<RepoErr> {
    reposorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>,
}

impl<RepoErr> ResponderEvaluacion<RepoErr> {
    pub fn new(reposorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>) -> Self {
        Self { reposorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), RespuestaError> for ResponderEvaluacion<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        Ok(self
            .reposorio
            .responder_evaluacion(RespuestaEvaluacion {
                id: RespuestaID::new(&in_.id)?,
                postulante_id: PreguntaID::new(&in_.postulante_id)?,
                evaluacion_id: in_.evaluacion_id,
                examen_id: in_.examen_id,
                pregunta_id: in_.pregunta_id,
                respuestas: in_.respuestas,
            })
            .await?)
    }
}
