use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioAgregarPregunta<Error>: Send + Sync {
    async fn agregar(
        &self,
        examen_id: ExamenID,
        preguntas: Vec<PreguntaEntity>,
    ) -> Result<(), Error>;
}
