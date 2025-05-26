use crate::controller::mongo_repository::MongoRepository;
use crate::controller::pregunta::mongo::constantes::EXAMEN_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::{doc, Bson, Document};
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::entity::pregunta_alternativas::PreguntaAlternativasProps;
use quizz_core::pregunta::domain::entity::pregunta_libre::PreguntaLibreProps;
use quizz_core::pregunta::domain::entity::pregunta_sola_respuesta::PreguntaSolaRespuestaProps;
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::pregunta::domain::service::tipo_pregunta::TipoDePregunta;
use quizz_core::pregunta::provider::repositorio::RepositorioAgregarPregunta;

pub struct PreguntaPorExamenMongo {
    client: web::Data<mongodb::Client>,
}

impl PreguntaPorExamenMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }

    fn tipo_pregunta_to_doc(&self, pregunta: TipoDePregunta) -> Document {
        match pregunta {
            TipoDePregunta::Alternativas(p) => self.pregunta_alternativas_to_doc(p),
            TipoDePregunta::Libre(p) => self.pregunta_libre_to_doc(p),
            TipoDePregunta::SolaRespuesta(p) => self.pregunta_sola_respuesta_to_doc(p),
        }
    }

    fn pregunta_alternativas_to_doc(&self, p: PreguntaEntity<PreguntaAlternativasProps>) -> Document {
        let mut alternativas_bson = Document::new();
        for (key, value) in &p.props.alternativas {
            alternativas_bson.insert(key, value);
        }

        doc! {
            "id": p.id().to_string(),
            "tipo": "alternativas",
            "contenido": p.contenido(),
            "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
            "alternativa_correcta": p.props.alternativa_correcta,
            "alternativas": alternativas_bson,
        }
    }

    fn pregunta_libre_to_doc(&self, p: PreguntaEntity<PreguntaLibreProps>) -> Document {
        doc! {
            "id": p.id().to_string(),
            "tipo": "libre",
            "contenido": p.contenido(),
            "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
        }
    }

    fn pregunta_sola_respuesta_to_doc(&self, p: PreguntaEntity<PreguntaSolaRespuestaProps>) -> Document {
        doc! {
            "id": p.id().to_string(),
            "tipo": "sola_respuesta",
            "contenido": p.contenido(),
            "imagen_ref": p.imagen_ref().map_or(Bson::Null, |s| Bson::String(s.to_string())),
            "respuesta_correcta": p.props.respuesta_correcta,
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
        todo!()
    }
}
