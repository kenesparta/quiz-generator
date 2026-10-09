use crate::controller::examen::mongo::constantes::EXAMEN_COLLECTION_NAME;
use crate::controller::mongo_repository::{MongoRepository, es_clave_duplicada};
use actix_web::web;
use mongodb::bson::doc;
use quizz_core::examen::domain::entity::examen::Examen;
use quizz_core::examen::domain::error::examen::ExamenError;
use quizz_core::examen::domain::error::examen::RepositorioError::PersistenciaNoFinalizada;
use quizz_core::examen::provider::repositorio::RepositorioExamenEscritura;
use tracing::error;

pub struct ExamenMongo {
    client: web::Data<mongodb::Database>,
}

impl ExamenMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        ExamenMongo { client }
    }
}

impl MongoRepository for ExamenMongo {
    fn get_collection_name(&self) -> &str {
        EXAMEN_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

impl RepositorioExamenEscritura for ExamenMongo {
    async fn guardar_examen(&self, examen: Examen) -> Result<(), ExamenError> {
        let documento = doc! {
            "_id": examen.id.value().uuid().to_string(),
            "titulo": examen.titulo.to_string(),
            "descripcion": examen.descripcion.to_string(),
            "instrucciones": examen.instrucciones.to_string(),
            "activo": examen.estado.to_string(),
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) if es_clave_duplicada(&e) => Err(ExamenError::ExamenRepositorioError(
                quizz_core::examen::domain::error::examen::RepositorioError::RegistroDuplicado,
            )),
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
