use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;

pub trait RepositorioAgregarPregunta: Send + Sync {
    fn agregar(
        &self,
        examen_id: ExamenID,
        lista_de_preguntas: ListaDePreguntas,
    ) -> impl Future<Output = Result<(), PreguntaError>> + Send;
}
