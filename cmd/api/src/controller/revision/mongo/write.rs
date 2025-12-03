use crate::controller::mongo_repository::MongoRepository;
use crate::controller::revision::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use quizz_core::respuesta::domain::entity::respuesta::Revision;
use quizz_core::respuesta::domain::entity::revision::ExamenRevision;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::RespositorioRealizarRevision;

pub struct RevisionEvaluacionMongo {
    client: web::Data<mongodb::Client>,
}

impl RevisionEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RevisionEvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RespositorioRealizarRevision<RespuestaError> for RevisionEvaluacionMongo {
    async fn realizar_revision(
        &self,
        revision_id: String,
        evaluacion_id: String,
        examenes: Vec<ExamenRevision>,
        estado: Revision
    ) -> Result<(), RespuestaError> {
        use mongodb::bson::doc;

        // First, verify the document exists and update the overall revision state
        let filter = doc! {
            "_id": &revision_id,
            "evaluacion._id": &evaluacion_id,
        };

        let update = doc! {
            "$set": {
                "revision": estado.to_string(),
            }
        };

        let result = self
            .get_collection()
            .update_one(filter.clone(), update, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if result.matched_count == 0 {
            return Err(RespuestaError::RespuestaNoEncontrada);
        }

        // Update each exam's observacion field
        for examen in examenes {
            let examen_filter = doc! {
                "_id": &revision_id,
                "evaluacion._id": &evaluacion_id,
                "evaluacion.examenes._id": &examen.examen_id,
            };

            let examen_update = doc! {
                "$set": {
                    "evaluacion.examenes.$[examen].observacion": &examen.observacion,
                }
            };

            let array_filters = vec![
                doc! { "examen._id": &examen.examen_id },
            ];

            let mut options = mongodb::options::UpdateOptions::default();
            options.array_filters = Some(array_filters);

            let examen_result = self
                .get_collection()
                .update_one(examen_filter, examen_update, Some(options))
                .await
                .map_err(|_| RespuestaError::DatabaseError)?;

            if examen_result.matched_count == 0 {
                return Err(RespuestaError::ExamenNotFound);
            }
        }

        Ok(())
    }
}
