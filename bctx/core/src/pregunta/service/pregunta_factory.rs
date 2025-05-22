use crate::pregunta::domain::entity::pregunta::{Pregunta, PreguntaEntity};
use crate::pregunta::domain::entity::pregunta_alternativas::PreguntaAlternativasProps;
use crate::pregunta::domain::entity::pregunta_libre::PreguntaLibreProps;
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
    ) -> impl Pregunta {
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
        respuesta_libre: String,
    ) -> impl Pregunta {
        let props = PreguntaLibreProps {
            contenido,
            imagen_ref,
            respuesta_libre,
        };

        PreguntaEntity::new(id, props)
    }
}
