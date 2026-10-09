use crate::mongo::psicologo::constantes::PSICOLOGO_COLLECTION_NAME;
use crate::mongo::repositorio::{CAMPO_ELIMINADO, MongoRepository, es_clave_duplicada};
use actix_web::web;
use mongodb::bson::doc;
use quizz_core::psicologo::domain::entity::psicologo::Psicologo;
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::domain::value_object::id::PsicologoID;
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

    async fn marcar_eliminado(&self, id: PsicologoID) -> Result<(), PsicologoError> {
        let resultado = self
            .get_collection()
            .update_one(
                doc! { "_id": id.to_string() },
                doc! { "$set": { CAMPO_ELIMINADO: true } },
            )
            .await
            .map_err(|e| {
                error!("Database error while marking psicologo as deleted: id={id}, error={e}");
                PsicologoError::PsicologoRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                )
            })?;

        // matched_count y no modified_count: si ya estaba marcado (un reintento), existe.
        if resultado.matched_count == 0 {
            return Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroNoEncontrado,
            ));
        }
        Ok(())
    }

    async fn eliminar_psicologo(&self, id: PsicologoID) -> Result<(), PsicologoError> {
        self.get_collection()
            .delete_one(doc! { "_id": id.to_string(), CAMPO_ELIMINADO: true })
            .await
            .map_err(|e| {
                error!("Database error while deleting psicologo: id={id}, error={e}");
                PsicologoError::PsicologoRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                )
            })?;
        Ok(())
    }
}
