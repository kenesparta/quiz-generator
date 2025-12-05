use crate::controller::mongo_repository::{MAIN_DATABASE_NAME, MongoRepository};
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use actix_web::web;
use async_trait::async_trait;
use mongodb;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Respuesta};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::{
    RepositorioRespuestaLectura, RespositorioRespuestaRevision,
};
use tracing::error;

pub struct RespuestaPorPostulanteMongo {
    client: web::Data<mongodb::Client>,
}

impl RespuestaPorPostulanteMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RespuestaPorPostulanteMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioRespuestaLectura<RespuestaError> for RespuestaPorPostulanteMongo {
    async fn obtener_por_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<Respuesta, RespuestaError> {
        let filter = doc! {
            "postulante_id": postulante_id.to_string()
        };

        let respuesta_doc = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(|e| {
                error!(
                    "Error finding respuesta by postulante_id {}: {}",
                    postulante_id, e
                );
                RespuestaError::RepositorioError
            })?;

        match respuesta_doc {
            Some(doc) => {
                let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                    error!("Error deserializing respuesta document: {}", e);
                    RespuestaError::RepositorioError
                })?;

                Ok(respuesta_dto.into())
            }

            None => Err(RespuestaError::RespuestaNoEncontrada),
        }
    }
}

pub struct RespuestaRevisionMongo {
    client: web::Data<mongodb::Client>,
}

impl RespuestaRevisionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RespuestaRevisionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RespositorioRespuestaRevision<RespuestaError> for RespuestaRevisionMongo {
    async fn obtener_respuesta_revision(
        &self,
        estado: Estado,
    ) -> Result<Vec<Respuesta>, RespuestaError> {
        let filter = doc! {
            "estado": estado.to_string()
        };

        let mut cursor = self
            .get_collection()
            .find(filter, None)
            .await
            .map_err(|e| {
                error!("Error finding respuestas by estado {}: {}", estado, e);
                RespuestaError::RepositorioError
            })?;

        let mut respuestas = Vec::new();

        while cursor.advance().await.map_err(|e| {
            error!("Error advancing cursor: {}", e);
            RespuestaError::RepositorioError
        })? {
            let doc = cursor.deserialize_current().map_err(|e| {
                error!("Error deserializing cursor: {}", e);
                RespuestaError::RepositorioError
            })?;
            let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                error!("Error deserializing respuesta document: {}", e);
                RespuestaError::RepositorioError
            })?;
            println!("respDTO: {:?}", respuesta_dto.revision);
            respuestas.push(respuesta_dto.into());
        }

        Ok(respuestas)
    }
}
