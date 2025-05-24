use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
use crate::pregunta::domain::entity::pregunta_alternativas::PreguntaAlternativasProps;
use crate::pregunta::domain::entity::pregunta_libre::PreguntaLibreProps;
use crate::pregunta::domain::entity::pregunta_sola_respuesta::PreguntaSolaRespuestaProps;

#[derive(Debug)]
pub enum TipoDePregunta {
    Alternativas(PreguntaEntity<PreguntaAlternativasProps>),
    Libre(PreguntaEntity<PreguntaLibreProps>),
    SolaRespuesta(PreguntaEntity<PreguntaSolaRespuestaProps>),
    // VerdaderoFalso,
    // Emparejamiento,
    // RellenarEspacios,
}
