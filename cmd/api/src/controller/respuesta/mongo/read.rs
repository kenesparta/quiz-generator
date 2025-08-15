use crate::controller::mongo_repository::MongoRepository;
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use actix_web::web;
use async_trait::async_trait;
use mongodb;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::respuesta::domain::entity::respuesta::Respuesta;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::RepositorioRespuestaLectura;
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
