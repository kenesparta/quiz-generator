use crate::controller::mongo_repository::MongoRepository;
use crate::controller::psicologo::mongo::constantes::PSICOLOGO_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use log::error;
use mongodb::bson::doc;
use quizz_core::psicologo::domain::entity::psicologo::Psicologo;
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::provider::repositorio::RepositorioPsicologoEscritura;

pub struct PsicologoMongo {
    client: web::Data<mongodb::Client>,
}

impl PsicologoMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        PsicologoMongo { client }
    }
}

impl MongoRepository for PsicologoMongo {
    fn get_collection_name(&self) -> &str {
        PSICOLOGO_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioPsicologoEscritura<PsicologoError> for PsicologoMongo {
    async fn registrar_psicologo(&self, psicologo: Psicologo) -> Result<(), PsicologoError> {
        let password = psicologo
            .password
            .ok_or(PsicologoError::PsicologoRepositorioError(
                RepositorioError::PasswordVacio,
            ))?;

        let documento = doc! {
            "_id": psicologo.id.value().uuid().to_string(),
            "nombre": psicologo.nombre,
            "primer_apellido": psicologo.primer_apellido,
            "segundo_apellido": psicologo.segundo_apellido,
            "documento": psicologo.documento,
            "especialidad": psicologo.especialidad,
            "password": password,
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Database error while registering psicologo: id={}, error={}",
                    psicologo.id, e
                );

                Err(PsicologoError::PsicologoRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
