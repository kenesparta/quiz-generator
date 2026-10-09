use crate::mongo::psicologo::constantes::PSICOLOGO_COLLECTION_NAME;
use crate::mongo::repositorio::{MongoRepository, es_clave_duplicada};
use actix_web::web;
use mongodb::bson::doc;
use quizz_core::psicologo::domain::entity::psicologo::Psicologo;
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::provider::repositorio::RepositorioPsicologoEscritura;
use tracing::error;

pub struct PsicologoMongo {
    client: web::Data<mongodb::Database>,
}

impl PsicologoMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        PsicologoMongo { client }
    }
}

impl MongoRepository for PsicologoMongo {
    fn get_collection_name(&self) -> &str {
        PSICOLOGO_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

impl RepositorioPsicologoEscritura for PsicologoMongo {
    async fn registrar_psicologo(&self, psicologo: Psicologo) -> Result<(), PsicologoError> {
        let password = psicologo
            .password
            .ok_or(PsicologoError::PsicologoRepositorioError(
                RepositorioError::PasswordVacio,
            ))?;

        let documento = doc! {
            "_id": psicologo.id.to_string(),
            "nombre": psicologo.nombre,
            "primer_apellido": psicologo.primer_apellido,
            "segundo_apellido": psicologo.segundo_apellido,
            "documento": psicologo.documento,
            "especialidad": psicologo.especialidad,
            "colegiatura": psicologo.colegiatura,
            "password": password,
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) if es_clave_duplicada(&e) => Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroDuplicado,
            )),
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
