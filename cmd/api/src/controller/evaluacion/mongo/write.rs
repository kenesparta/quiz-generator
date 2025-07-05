use crate::controller::evaluacion::mongo::constantes::EVALUACION_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_core::evaluacion::domain::entity::evaluacion::Evaluacion;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::PersistenciaNoFinalizada;
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
        todo!()
    }
}
