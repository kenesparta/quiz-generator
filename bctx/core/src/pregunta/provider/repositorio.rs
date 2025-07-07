use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioAgregarPregunta<Error>: Send + Sync {
    async fn agregar(
        &self,
        examen_id: ExamenID,
        lista_de_preguntas: ListaDePreguntas,
    ) -> Result<(), Error>;
}
