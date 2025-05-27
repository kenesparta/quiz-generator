use crate::controller::mongo_repository::MongoRepository;
use crate::controller::pregunta::mongo::constantes::EXAMEN_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::{Bson, Document, doc};
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::entity::pregunta_alternativas::PreguntaAlternativasProps;
use quizz_core::pregunta::domain::entity::pregunta_libre::PreguntaLibreProps;
use quizz_core::pregunta::domain::entity::pregunta_sola_respuesta::PreguntaSolaRespuestaProps;
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::pregunta::domain::error::pregunta::RepositorioError::{
    ActualizacionNoFinalizada, PersistenciaNoFinalizada,
};
use quizz_core::pregunta::domain::service::tipo_pregunta::TipoDePregunta;
use quizz_core::pregunta::provider::repositorio::RepositorioAgregarPregunta;
use tracing::log::{error, info};

pub struct PreguntaPorExamenMongo {
    client: web::Data<mongodb::Client>,
}

impl PreguntaPorExamenMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }

    fn pregunta_to_bson(&self, pregunta: &TipoDePregunta) -> Result<Bson, PreguntaError> {
        match pregunta {
            TipoDePregunta::Alternativas(al) => {
                let mut alternativas_bson = Document::new();

                // for (key, value) in &p.props.alternativas {
                //     alternativas_bson.insert(key, value);
                // }
                let doc = doc! {
                    "id":  al.id.to_string(),
                    "tipo": al.tipo_pregunta(),
                    "contenido": p.contenido(),
                    "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
                    "alternativa_correcta": &p.props.alternativa_correcta,
                    "alternativas": alternativas_bson,
                };
                Ok(Bson::Document(doc))
            }
            TipoDePregunta::Libre(p) => {
                let doc = doc! {
                    "id": p.id().to_string(),
                    "tipo": "libre",
                    "contenido": p.contenido(),
                    "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
                };
                Ok(Bson::Document(doc))
            }
            TipoDePregunta::SolaRespuesta(p) => {
                let doc = doc! {
                    "id": p.id().to_string(),
                    "tipo": "sola_respuesta",
                    "contenido": p.contenido(),
                    "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
                    "respuesta_correcta": &p.props.respuesta_correcta,
                };
                Ok(Bson::Document(doc))
            }
        }
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
        preguntas: Vec<TipoDePregunta>,
    ) -> Result<(), PreguntaError> {
        let examen_filter = doc! {
            "id": examen_id.to_string()
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

        let mut preguntas_bson = Vec::new();
        for pregunta in preguntas {
            preguntas_bson.push(self.pregunta_to_bson(&pregunta)?);
        }

        let update = doc! {
            "$push": {
                "preguntas": {
                    "$each": preguntas_bson
                }
            }
        };

        let examen_filter = doc! {
            "id": examen_id.to_string()
        };
        self.get_collection()
            .update_one(examen_filter, update, None)
            .await
            .map_err(|e| {
                error!("Error updating exam with questions: {}", e);
                PreguntaError::PreguntaRepositorioError(ActualizacionNoFinalizada)
            })?;

        info!("Questions added successfully to exam {}", examen_id);
        Ok(())
    }
}
