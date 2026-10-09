use crate::controller::mongo_repository::{MongoRepository, es_clave_duplicada};
use crate::controller::postulante::mongo::constantes::POSTULANTE_COLLECTION_NAME;
use actix_web::web;
use mongodb::bson::doc;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use tracing::error;

pub struct PostulanteMongo {
    client: web::Data<mongodb::Database>,
}

impl PostulanteMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        PostulanteMongo { client }
    }
}

impl MongoRepository for PostulanteMongo {
    fn get_collection_name(&self) -> &str {
        POSTULANTE_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

impl RepositorioPostulanteEscritura for PostulanteMongo {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), PostulanteError> {
        let password = postulante
            .password
            .ok_or(PostulanteError::PostulanteRepositorioError(
                RepositorioError::PasswordVacio,
            ))?
            .value();

        let documento = doc! {
            "_id": postulante.id.to_string(),
            "documento": postulante.documento.to_string(),
            "nombre": postulante.nombre_completo.nombre(),
            "primer_apellido": postulante.nombre_completo.primer_apellido(),
            "segundo_apellido": postulante.nombre_completo.segundo_apellido(),
            "fecha_nacimiento": postulante.fecha_nacimiento.to_string(),
            "grado_instruccion": postulante.grado_instruccion.to_string(),
            "genero": postulante.genero.to_string(),
            "password": password,
            "fecha_registro": postulante.fecha_registro.to_string(),
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) if es_clave_duplicada(&e) => Err(PostulanteError::PostulanteRepositorioError(
                RepositorioError::RegistroDuplicado,
            )),
            Err(e) => {
                error!(
                    "Database error while registering postulante: id={}, error={}",
                    postulante.id, e
                );

                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn actualizar_postulante(&self, postulante: Postulante) -> Result<(), PostulanteError> {
        let filter = doc! {
            "_id": postulante.id.to_string(),
        };

        let fecha_actualizacion =
            quizz_common::domain::value_objects::zona_horaria::formatear_rfc3339(
                &quizz_common::domain::value_objects::zona_horaria::ahora_lima(),
            );

        let update = doc! {
            "$set": {
                "nombre": postulante.nombre_completo.nombre(),
                "primer_apellido": postulante.nombre_completo.primer_apellido(),
                "segundo_apellido": postulante.nombre_completo.segundo_apellido(),
                "fecha_nacimiento": postulante.fecha_nacimiento.to_string(),
                "grado_instruccion": postulante.grado_instruccion.to_string(),
                "genero": postulante.genero.to_string(),
                "fecha_actualizacion": fecha_actualizacion,
            }
        };

        match self.get_collection().update_one(filter, update).await {
            Ok(result) => {
                if result.matched_count == 0 {
                    return Err(PostulanteError::PostulanteRepositorioError(
                        RepositorioError::RegistroNoEncontrado,
                    ));
                }
                Ok(())
            }
            Err(e) => {
                error!(
                    "Database error while updating postulante: id={}, error={}",
                    postulante.id, e
                );

                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn eliminar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<(), PostulanteError> {
        let filter = doc! {
            "_id": postulante_id.to_string(),
        };

        match self.get_collection().delete_one(filter).await {
            Ok(result) => {
                if result.deleted_count == 0 {
                    return Err(PostulanteError::PostulanteRepositorioError(
                        RepositorioError::RegistroNoEncontrado,
                    ));
                }
                Ok(())
            }
            Err(e) => {
                error!(
                    "Database error while deleting postulante: id={}, error={}",
                    postulante_id, e
                );

                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
