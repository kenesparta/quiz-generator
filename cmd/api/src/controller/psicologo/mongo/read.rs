use crate::controller::mongo_repository::MongoRepository;
use crate::controller::psicologo::mongo::constantes::PSICOLOGO_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use log::error;
use mongodb::bson::doc;
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::provider::repositorio::{PsicologoInfo, RepositorioPsicologoLectura};

pub struct PsicologoReadMongo {
    client: web::Data<mongodb::Client>,
}

impl PsicologoReadMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for PsicologoReadMongo {
    fn get_collection_name(&self) -> &str {
        PSICOLOGO_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioPsicologoLectura<PsicologoError> for PsicologoReadMongo {
    async fn obtener_psicologo_por_id(&self, id: String) -> Result<PsicologoInfo, PsicologoError> {
        let filter = doc! { "_id": &id };

        let doc = self.get_collection().find_one(filter).await.map_err(|e| {
            error!("Error finding psicologo by id {}: {}", id, e);
            PsicologoError::PsicologoRepositorioError(RepositorioError::LecturaNoFinalizada)
        })?;

        match doc {
            Some(doc) => {
                let nombre = doc.get_str("nombre").unwrap_or_default().to_string();
                let primer_apellido = doc
                    .get_str("primer_apellido")
                    .unwrap_or_default()
                    .to_string();
                let segundo_apellido = doc
                    .get_str("segundo_apellido")
                    .unwrap_or_default()
                    .to_string();
                let colegiatura = doc.get_str("colegiatura").unwrap_or_default().to_string();

                Ok(PsicologoInfo {
                    nombre,
                    primer_apellido,
                    segundo_apellido,
                    colegiatura,
                })
            }
            None => Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroNoEncontrado,
            )),
        }
    }
}
