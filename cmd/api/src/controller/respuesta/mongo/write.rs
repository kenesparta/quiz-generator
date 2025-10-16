use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::mongo_repository::MongoRepository;
use crate::controller::postulante::mongo::write::PostulanteMongo;
use crate::controller::respuesta::dto::{EvaluacionMongoDTO, RespuestaMongoDTO};
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use chrono::Utc;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::respuesta::domain::entity::respuesta::RespuestaEvaluacion;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use quizz_core::respuesta::provider::repositorio::RepositorioRespuestaEscritura;

pub struct RespuestaEvaluacionMongo {
    client: web::Data<mongodb::Client>,
    repositorio_evaluacion: EvaluacionMongo,
    reposiorio_postulante: PostulanteMongo,
}

impl RespuestaEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        let repositorio_evaluacion = EvaluacionMongo::new(client.clone());
        let reposiorio_postulante = PostulanteMongo::new(client.clone());
        Self {
            client,
            repositorio_evaluacion,
            reposiorio_postulante,
        }
    }
}

impl MongoRepository for RespuestaEvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioRespuestaEscritura<RespuestaError> for RespuestaEvaluacionMongo {
    async fn asignar_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> Result<(), RespuestaError> {
        let existing_respuesta = self
            .get_collection()
            .find_one(
                doc! {
                    "evaluacion._id": evaluacion_id.to_string(),
                    "postulante_id": postulante_id.to_string(),
                },
                None,
            )
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if existing_respuesta.is_some() {
            return Err(RespuestaError::EvaluacionAlreadyAssigned);
        }

        let postulante_exists = self
            .reposiorio_postulante
            .get_collection()
            .find_one(doc! { "_id": postulante_id.to_string() }, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if postulante_exists.is_none() {
            return Err(RespuestaError::PostulanteRespuestaNotFound);
        }

        let evaluacion_doc = self
            .repositorio_evaluacion
            .get_collection()
            .find_one(doc! { "_id": evaluacion_id.to_string() }, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        let evaluacion_document =
            evaluacion_doc.ok_or_else(|| RespuestaError::EvaluacionRespuestaNotFound)?;

        let evaluacion: EvaluacionMongoDTO =
            bson::from_document(evaluacion_document).map_err(|_| RespuestaError::DatabaseError)?;

        let respuesta_dto = RespuestaMongoDTO {
            id: RespuestaID::new_v4().to_string(),
            evaluacion,
            postulante_id: postulante_id.to_string(),
            fecha_tiempo_inicio: Utc::now().to_rfc3339(),
            fecha_tiempo_fin: String::new(),
        };

        let respuesta_doc =
            bson::to_document(&respuesta_dto).map_err(|_| RespuestaError::DatabaseError)?;

        self.get_collection()
            .insert_one(respuesta_doc, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }

    async fn responder_evaluacion(
        &self,
        respuesta_evaluacion: RespuestaEvaluacion,
    ) -> Result<(), RespuestaError> {
        let filter = doc! {
            "_id": &respuesta_evaluacion.id.to_string(),
            "evaluacion._id": &respuesta_evaluacion.evaluacion_id,
            "evaluacion.examenes._id": &respuesta_evaluacion.examen_id,
            "evaluacion.examenes.preguntas._id": &respuesta_evaluacion.pregunta_id
        };

        // Build the update operation using positional operators
        // $[examen] and $[pregunta] are array filters to identify the specific elements
        let update = doc! {
            "$set": {
                "evaluacion.examenes.$[examen].preguntas.$[pregunta].respuestas": &respuesta_evaluacion.respuestas
            }
        };

        // Define array filters to identify which exam and question to update
        let array_filters = vec![
            doc! { "examen._id": &respuesta_evaluacion.examen_id },
            doc! { "pregunta._id": &respuesta_evaluacion.pregunta_id }
        ];

        // Set up the update options with array filters
        let mut options = mongodb::options::UpdateOptions::default();
        options.array_filters = Some(array_filters);

        // Execute the update operation
        let _result = self
            .get_collection()
            .update_one(filter, update, Some(options))
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }
}
