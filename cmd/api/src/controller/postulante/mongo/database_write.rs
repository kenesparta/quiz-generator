use crate::controller::postulante::mongo::constantes::{
    MAIN_DATABASE_NAME, POSTULANTE_COLLECTION_NAME,
};
use actix_web::web;
use async_trait::async_trait;
use mongodb::Collection;
use mongodb::bson::{Document, doc};
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use tracing::log::error;

pub struct PostulanteMongo {
    client: web::Data<mongodb::Client>,
}

impl PostulanteMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        PostulanteMongo { client }
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client
            .database(MAIN_DATABASE_NAME)
            .collection::<Document>(POSTULANTE_COLLECTION_NAME)
    }
}

#[async_trait]
impl RepositorioPostulanteEscritura<PostulanteError> for PostulanteMongo {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), PostulanteError> {
        let client = self.client.as_ref();
        let password = postulante
            .password
            .ok_or(PostulanteError::PostulanteRepositorioError(
                RepositorioError::PasswordVacio,
            ))?
            .value();

        let documento = doc! {
            "id": postulante.id.value().uuid().to_string(),
            "documento": postulante.documento.to_string(),
            "nombre": postulante.nombre_completo.nombre(),
            "primer_apellido": postulante.nombre_completo.primer_apellido(),
            "segundo_apellido": postulante.nombre_completo.segundo_apellido(),
            "fecha_nacimiento": postulante.fecha_nacimiento.to_string(),
            "grado_instruccion": postulante.grado_instruccion.to_string(),
            "genero": postulante.genero.to_string(),
            "password": password,
        };

        match self.get_collection().insert_one(documento, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Database error while registering postulante: id={}, documento={}, error={}",
                    postulante.id, postulante.documento, e
                );

                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn actualizar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<(), PostulanteError> {
        todo!()
    }

    async fn eliminar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<(), PostulanteError> {
        let filter = doc! {
            "id": postulante_id.value().uuid().to_string(),
        };

        match self.get_collection().delete_one(filter, None).await {
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
