use crate::controller::mongo_repository::MongoRepository;
use crate::controller::pregunta::mongo::constantes::EXAMEN_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::{Bson, Document, doc};
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::pregunta::domain::error::pregunta::RepositorioError::{
    ActualizacionNoFinalizada, PersistenciaNoFinalizada,
};
use quizz_core::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use quizz_core::pregunta::provider::repositorio::RepositorioAgregarPregunta;
use tracing::log::{error, info};

pub struct PreguntaPorExamenMongo {
    client: web::Data<mongodb::Client>,
}

impl PreguntaPorExamenMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for PreguntaPorExamenMongo {
    fn get_collection_name(&self) -> &str {
        EXAMEN_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioAgregarPregunta<PreguntaError> for PreguntaPorExamenMongo {
    async fn agregar(
        &self,
        examen_id: ExamenID,
        lista_de_preguntas: ListaDePreguntas,
    ) -> Result<(), PreguntaError> {
        let examen_filter = doc! {
            "_id": examen_id.to_string()
        };

        let examen_exists = self
            .get_collection()
            .find_one(examen_filter, None)
            .await
            .map_err(|e| {
                error!("Error checking if exam exists: {}", e);
                PreguntaError::PreguntaRepositorioError(PersistenciaNoFinalizada)
            })?;

        if examen_exists.is_none() {
            return Err(PreguntaError::PreguntaRepositorioError(
                PersistenciaNoFinalizada,
            ));
        }

        let preguntas_bson = preguntas_to_bson(&lista_de_preguntas.preguntas());

        let update = doc! {
            "$push": {
                "preguntas": {
                    "$each": preguntas_bson
                }
            }
        };

        self.get_collection()
            .update_one(
                doc! {
                    "_id": examen_id.to_string()
                },
                update,
                None,
            )
            .await
            .map_err(|e| {
                error!("Error updating exam with questions: {}", e);
                PreguntaError::PreguntaRepositorioError(ActualizacionNoFinalizada)
            })?;

        info!("Questions added successfully to exam {}", examen_id);
        Ok(())
    }
}

fn preguntas_to_bson(preguntas: &[PreguntaEntity]) -> Vec<Bson> {
    preguntas
        .iter()
        .map(|pregunta| {
            let mut document = doc! {
                "_id": pregunta.id.to_string(),
                "contenido": pregunta.contenido.clone(),
                "etiqueta": pregunta.etiqueta.to_string(),
                "tipo_de_pregunta": pregunta.tipo_de_pregunta.to_string()
            };

            if let Some(ref imagen) = pregunta.imagen_ref {
                document.insert("imagen_ref", imagen.clone());
            }

            let ref alternativas = pregunta.alternativas;
            let alternativas_doc = alternativas
                .iter()
                .map(|(key, value)| (key.to_string(), Bson::String(value.clone())))
                .collect::<Document>();

            document.insert("alternativas", alternativas_doc);

            let ref puntaje = pregunta.puntaje;
            let puntaje_doc = puntaje
                .iter()
                .map(|(key, value)| (key.to_string(), Bson::Int32(*value as i32)))
                .collect::<Document>();

            document.insert("puntaje", puntaje_doc);

            Bson::Document(document)
        })
        .collect()
}
