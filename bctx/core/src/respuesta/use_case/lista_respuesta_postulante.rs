use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioListaRespuestaPostulante;

pub struct InputData {
    pub postulante_id: String,
}

pub struct OutputData {
    pub respuesta_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado: Estado,
}

pub struct ListaRespuestaPostulante<R> {
    repo: R,
}

impl<R: RepositorioListaRespuestaPostulante> ListaRespuestaPostulante<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn ejecutar(&self, input: InputData) -> Result<Vec<OutputData>, RespuestaError> {
        let postulante_id = PostulanteID::new(&input.postulante_id)?;

        let respuestas = self
            .repo
            .obtener_respuestas_por_postulante(postulante_id)
            .await?;

        Ok(respuestas)
    }
}
