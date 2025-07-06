use crate::controller::evaluacion::mongo::constantes::EVALUACION_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_core::evaluacion::domain::entity::evaluacion::Evaluacion;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::{
    EvaluacionNoExiste, PersistenciaNoFinalizada,
};
use quizz_core::evaluacion::provider::repositorio::RepositorioEvaluacionEscritura;
use quizz_core::evaluacion::value_object::examen_id::ExamenIDs;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use tracing::log::error;

pub struct EvaluacionMongo {
    client: web::Data<mongodb::Client>,
}

impl EvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        EvaluacionMongo { client }
    }
}

impl MongoRepository for EvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        EVALUACION_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioEvaluacionEscritura<EvaluacionError> for EvaluacionMongo {
    async fn guardar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), EvaluacionError> {
        let documento = doc! {
            "_id": evaluacion.id.to_string(),
            "nombre": evaluacion.nombre,
            "descripcion": evaluacion.descripcion,
            "estado": evaluacion.estado.to_string(),
        };

        match self.get_collection().insert_one(documento, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error al guardar evaluacion: {e}");
                Err(EvaluacionError::EvaluacionRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn agregar_examen(
        &self,
        evaluacion_id: EvaluacionID,
        examen_ids: ExamenIDs,
    ) -> Result<(), EvaluacionError> {
        let evaluacion_exists = self
            .get_collection()
            .find_one(
                doc! {
                    "_id": evaluacion_id.to_string()
                },
                None,
            )
            .await
            .map_err(|e| {
                error!("Error checking if exam exists: {}", e);
                EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
            })?;

        if evaluacion_exists.is_none() {
            return Err(EvaluacionError::EvaluacionRepositorioError(
                EvaluacionNoExiste,
            ));
        }

        let examen_ids_strings: Vec<String> = examen_ids
            .examen_ids
            .iter()
            .map(|id| id.uuid().to_string())
            .collect();

        let update = doc! {
            "$addToSet": {
                "examenes": {
                    "$each": examen_ids_strings
                }
            }
        };

        match self
            .get_collection()
            .update_one(
                doc! {
                    "_id": evaluacion_id.to_string()
                },
                update,
                None,
            )
            .await
        {
            Ok(result) => {
                if result.matched_count == 0 {
                    error!("No evaluation found with ID: {}", evaluacion_id.to_string());
                    return Err(EvaluacionError::EvaluacionRepositorioError(
                        EvaluacionNoExiste,
                    ));
                }
                Ok(())
            }
            Err(e) => {
                error!("Error updating evaluation with exam IDs: {}", e);
                Err(EvaluacionError::EvaluacionRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
