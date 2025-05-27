use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
use std::collections::HashMap;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::etiqueta::Etiqueta;

pub trait Pregunta {
    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError>;
}

pub struct PreguntaWrapper;

impl PreguntaWrapper {
   pub fn pregunta_entity(&self,         id: String,
                          contenido: String,
                          etiqueta: String,
                          tipo_de_pregunta: String,
                          imagen_ref: Option<String>,
                          alternativas: HashMap<String, String>,
                          puntos: HashMap<String, u32>,) -> Result<PreguntaEntity, PreguntaError> {
       unimplemented!()
   }
}

#[derive(Debug, Clone)]
pub struct PreguntaEntityList {
    pub pregunta: Vec<PreguntaEntity>,
}

pub struct PreguntaAlternativas;
pub struct PreguntaLibre;
pub struct PreguntaSolaRespuesta;
pub struct PreguntaSiNo;
