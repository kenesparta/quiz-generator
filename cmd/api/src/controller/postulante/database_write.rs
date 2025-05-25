use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::{Document, doc};
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use tracing::log::error;

pub struct PostulanteMongo {
    client: web::Data<mongodb::Client>,
    database_name: String,
    collection_name: String,
}

impl PostulanteMongo {
    pub fn new(
        client: web::Data<mongodb::Client>,
        database_name: String,
        collection_name: String,
    ) -> Self {
        PostulanteMongo {
            client,
            database_name,
            collection_name,
        }
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

        let collection = client
            .database(&self.database_name)
            .collection::<Document>(&self.collection_name);

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

        match collection.insert_one(documento, None).await {
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
        let client = self.client.as_ref();
        let collection = client
            .database(&self.database_name)
            .collection::<Document>(&self.collection_name);

        let filter = doc! {
            "id": postulante_id.value().uuid().to_string(),
        };

        match collection.delete_one(filter, None).await {
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
