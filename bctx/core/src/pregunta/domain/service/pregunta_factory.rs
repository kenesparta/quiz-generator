use crate::pregunta::domain::entity::pregunta::{Pregunta, PreguntaEntity};
use crate::pregunta::domain::entity::pregunta_alternativas::PreguntaAlternativasProps;
use crate::pregunta::domain::entity::pregunta_libre::PreguntaLibreProps;
use crate::pregunta::domain::entity::pregunta_sola_respuesta::PreguntaSolaRespuestaProps;
use crate::pregunta::domain::value_object::id::PreguntaID;
use std::collections::HashMap;

pub struct PreguntaFactory;

impl PreguntaFactory {
    pub fn pregunta_alternativas(
        id: PreguntaID,
        contenido: String,
        imagen_ref: Option<String>,
        alternativa_correcta: String,
        alternativas: HashMap<String, String>,
    ) -> PreguntaEntity<PreguntaAlternativasProps> {
        let props = PreguntaAlternativasProps {
            contenido,
            imagen_ref,
            alternativa_correcta,
            alternativas,
        };

        PreguntaEntity::new(id, props)
    }

    pub fn pregunta_libre(
        id: PreguntaID,
        contenido: String,
        imagen_ref: Option<String>,
    ) -> PreguntaEntity<PreguntaLibreProps> {
        let props = PreguntaLibreProps {
            contenido,
            imagen_ref,
        };

        PreguntaEntity::new(id, props)
    }

    pub fn pregunta_sola_respuesta(
        id: PreguntaID,
        contenido: String,
        imagen_ref: Option<String>,
        respuesta_correcta: String,
    ) -> PreguntaEntity<PreguntaSolaRespuestaProps> {
        let props = PreguntaSolaRespuestaProps {
            contenido,
            imagen_ref,
            respuesta_correcta,
        };

        PreguntaEntity::new(id, props)
    }
}
