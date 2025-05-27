use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::service::pregunta_factory::{PreguntaEntityList, PreguntaFactory};
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
    pub preguntas: PreguntaEntityList,
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
        // let preguntas: Vec<TipoDePregunta> = in_
        //     .preguntas
        //     .into_iter()
        //     .map(|r| match r {
        //         PreguntaRawData::Alternativas {
        //             id,
        //             contenido,
        //             imagen_ref,
        //             alternativa_correcta,
        //             alternativas,
        //         } => {
        //             let id = PreguntaID::new(&id)?;
        //             Ok(Alternativas(PreguntaFactory::pregunta_alternativas(
        //                 id,
        //                 contenido,
        //                 imagen_ref,
        //                 alternativa_correcta,
        //                 alternativas,
        //             )))
        //         }
        //         PreguntaRawData::Libre {
        //             id,
        //             contenido,
        //             imagen_ref,
        //         } => {
        //             let id = PreguntaID::new(&id)?;
        //             Ok(Libre(PreguntaFactory::pregunta_libre(
        //                 id, contenido, imagen_ref,
        //             )))
        //         }
        //         PreguntaRawData::SolaRespuesta {
        //             id,
        //             contenido,
        //             imagen_ref,
        //             respuesta_correcta,
        //         } => {
        //             let id = PreguntaID::new(&id)?;
        //             Ok(SolaRespuesta(PreguntaFactory::pregunta_sola_respuesta(
        //                 id,
        //                 contenido,
        //                 imagen_ref,
        //                 respuesta_correcta,
        //             )))
        //         }
        //     })
        //     .collect::<Result<Vec<TipoDePregunta>, PreguntaError>>()?;

        // self.reposotorio.agregar(examen_id, preguntas).await?;
        Ok(())
    }
}
