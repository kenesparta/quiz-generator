use crate::controller::mongo_repository::MongoRepository;
use crate::controller::respuesta::mongo::RespuestaMongo;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Respuesta};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use quizz_core::respuesta::provider::repositorio::RepositorioObtenerRevisionPorId;
use tracing::error;

impl RepositorioObtenerRevisionPorId for RespuestaMongo {
    async fn obtener_revision_por_id(
        &self,
        respuesta_id: &RespuestaID,
    ) -> Result<Respuesta, RespuestaError> {
        let filter = doc! {
            "_id": respuesta_id.to_string(),
            "estado": Estado::Finalizado.to_string(),
        };

        let doc = self.get_collection().find_one(filter).await.map_err(|e| {
            error!("Error finding revision by id {}: {}", respuesta_id, e);
            RespuestaError::RepositorioError
        })?;

        match doc {
            Some(doc) => {
                let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                    error!("Error deserializing revision document: {}", e);
                    RespuestaError::RepositorioError
                })?;
                respuesta_dto.a_dominio()
            }
            None => Err(RespuestaError::RespuestaNoEncontrada),
        }
    }
}
