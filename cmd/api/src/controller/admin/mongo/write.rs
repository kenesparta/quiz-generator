use crate::controller::admin::mongo::constantes::ADMIN_COLLECTION_NAME;
use crate::controller::mongo_repository::{MongoRepository, es_clave_duplicada};
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_core::admin::domain::entity::admin::Admin;
use quizz_core::admin::domain::error::admin::{AdminError, RepositorioError};
use quizz_core::admin::provider::repositorio::RepositorioAdminEscritura;
use tracing::error;

pub struct AdminMongo {
    client: web::Data<mongodb::Database>,
}

impl AdminMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        AdminMongo { client }
    }
}

impl MongoRepository for AdminMongo {
    fn get_collection_name(&self) -> &str {
        ADMIN_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

#[async_trait]
impl RepositorioAdminEscritura<AdminError> for AdminMongo {
    async fn registrar_admin(&self, admin: Admin) -> Result<(), AdminError> {
        let password = admin.password.ok_or(AdminError::AdminRepositorioError(
            RepositorioError::PasswordVacio,
        ))?;

        let documento = doc! {
            "_id": admin.id.value().uuid().to_string(),
            "nombre": admin.nombre,
            "primer_apellido": admin.primer_apellido,
            "segundo_apellido": admin.segundo_apellido,
            "documento": admin.documento,
            "password": password,
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) if es_clave_duplicada(&e) => Err(AdminError::AdminRepositorioError(
                RepositorioError::RegistroDuplicado,
            )),
            Err(e) => {
                error!(
                    "Database error while registering admin: id={}, error={}",
                    admin.id, e
                );

                Err(AdminError::AdminRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
