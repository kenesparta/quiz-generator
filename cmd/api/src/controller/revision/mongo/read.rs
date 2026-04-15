use crate::controller::mongo_repository::MongoRepository;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use crate::controller::revision::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Respuesta};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::RepositorioObtenerRevisionPorId;
use tracing::error;

pub struct RevisionReadMongo {
    client: web::Data<mongodb::Client>,
}

impl RevisionReadMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RevisionReadMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioObtenerRevisionPorId<RespuestaError> for RevisionReadMongo {
    async fn obtener_revision_por_id(
        &self,
        revision_id: String,
    ) -> Result<Respuesta, RespuestaError> {
        let filter = doc! {
            "_id": &revision_id,
            "estado": Estado::Finalizado.to_string(),
        };

        let doc = self
            .get_collection()
            .find_one(filter)
            .await
            .map_err(|e| {
                error!("Error finding revision by id {}: {}", revision_id, e);
                RespuestaError::RepositorioError
            })?;

        match doc {
            Some(doc) => {
                let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                    error!("Error deserializing revision document: {}", e);
                    RespuestaError::RepositorioError
                })?;
                Ok(respuesta_dto.into())
            }
            None => Err(RespuestaError::RespuestaNoEncontrada),
        }
    }
}
