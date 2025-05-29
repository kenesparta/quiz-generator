use crate::controller::examen::mongo::constantes::EXAMEN_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_core::examen::domain::entity::examen::Examen;
use quizz_core::examen::domain::error::examen::ExamenError;
use quizz_core::examen::domain::error::examen::RepositorioError::PersistenciaNoFinalizada;
use quizz_core::examen::provider::repositorio::RepositorioExamenEscritura;
use tracing::log::error;

pub struct ExamenMongo {
    client: web::Data<mongodb::Client>,
}

impl ExamenMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        ExamenMongo { client }
    }
}

impl MongoRepository for ExamenMongo {
    fn get_collection_name(&self) -> &str {
        EXAMEN_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioExamenEscritura<ExamenError> for ExamenMongo {
    async fn guardar_examen(&self, examen: Examen) -> Result<(), ExamenError> {
        let documento = doc! {
            "id": examen.id.value().uuid().to_string(),
            "titulo": examen.titulo.to_string(),
            "descripcion": examen.descripcion.to_string(),
            "activo": examen.activo,
        };

        match self.get_collection().insert_one(documento, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Database error while registering examen: id={}, titulo={}, error={}",
                    examen.id, examen.titulo, e
                );

                Err(ExamenError::ExamenRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
