use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioAgregarPregunta<Error>: Send + Sync {
    async fn agregar(&self, examen_id: ExamenID, preguntas: ListaDePreguntas) -> Result<(), Error>;
}
