use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::service::pregunta_factory::PreguntaFactory;
use crate::pregunta::domain::service::tipo_pregunta::TipoDePregunta;
use crate::pregunta::domain::service::tipo_pregunta::TipoDePregunta::{
    Alternativas, Libre, SolaRespuesta,
};
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::provider::repositorio::RepositorioAgregarPregunta;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InputData {
    pub examen_id: String,
    pub preguntas: Vec<PreguntaRawData>,
}

#[derive(Debug, Clone)]
pub enum PreguntaRawData {
    Alternativas {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
        alternativa_correcta: String,
        alternativas: HashMap<String, String>,
    },
    Libre {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
    },
    SolaRespuesta {
        id: String,
        contenido: String,
        imagen_ref: Option<String>,
        respuesta_correcta: String,
    },
}

pub struct AgregarPreguntas<RepoErr> {
    reposotorio: Box<dyn RepositorioAgregarPregunta<RepoErr>>,
}

impl<RepoErr> AgregarPreguntas<RepoErr> {
    pub fn new(reposotorio: Box<dyn RepositorioAgregarPregunta<RepoErr>>) -> Self {
        Self { reposotorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), PreguntaError> for AgregarPreguntas<RepoErr>
where
    PreguntaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), PreguntaError> {
        let examen_id = ExamenID::new(&in_.examen_id)?;
        // let mut preguntas_concretas: Vec<Box<dyn Pregunta>> = Vec::new();
        let mut preguntas: Vec<TipoDePregunta> = Vec::new();

        // Process each TipoDePregunta and convert it to a concrete Pregunta
        for tipo_pregunta in in_.preguntas {
            match tipo_pregunta {
                PreguntaRawData::Alternativas {
                    id,
                    contenido,
                    imagen_ref,
                    alternativa_correcta,
                    alternativas,
                } => {
                    // Use PreguntaFactory to create a multiple-choice question
                    let id = PreguntaID::new(&id)?;
                    let pregunta = PreguntaFactory::pregunta_alternativas(
                        id,
                        contenido.to_string(),
                        imagen_ref.map(|s| s.to_string()),
                        alternativa_correcta,
                        alternativas,
                    );

                    preguntas.push(Alternativas(pregunta));
                }
                PreguntaRawData::Libre {
                    id,
                    contenido,
                    imagen_ref,
                } => {
                    let id = PreguntaID::new(&id)?;
                    let pregunta = PreguntaFactory::pregunta_libre(
                        id,
                        contenido.to_string(),
                        imagen_ref.map(|s| s.to_string()),
                    );

                    preguntas.push(Libre(pregunta));
                }
                PreguntaRawData::SolaRespuesta {
                    id,
                    contenido,
                    imagen_ref,
                    respuesta_correcta
                } => {
                    let id = PreguntaID::new(&id)?;
                    let pregunta = PreguntaFactory::pregunta_sola_respuesta(
                        id,
                        contenido.to_string(),
                        imagen_ref.map(|s| s.to_string()),
                        respuesta_correcta
                    );
                    preguntas.push(SolaRespuesta(pregunta));
                }
            }
        }

        // Pass the concrete Pregunta objects to the repository
        self.reposotorio.agregar(examen_id, preguntas).await?;
        Ok(())
    }
}
