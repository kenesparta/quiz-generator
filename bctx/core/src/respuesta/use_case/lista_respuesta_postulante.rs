use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioListaRespuestaPostulante;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub postulante_id: String,
}

pub struct OutputData {
    pub respuesta_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado: String,
}

pub struct ListaRespuestaPostulante<RepoErr> {
    repo: Box<dyn RepositorioListaRespuestaPostulante<RepoErr>>,
}

impl<RepoErr> ListaRespuestaPostulante<RepoErr> {
    pub fn new(repo: Box<dyn RepositorioListaRespuestaPostulante<RepoErr>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, Vec<OutputData>, RespuestaError>
    for ListaRespuestaPostulante<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, input: InputData) -> Result<Vec<OutputData>, RespuestaError> {
        let postulante_id = PostulanteID::new(&input.postulante_id)?;

        let respuestas = self
            .repo
            .obtener_respuestas_por_postulante(postulante_id)
            .await?;

        Ok(respuestas)
    }
}
