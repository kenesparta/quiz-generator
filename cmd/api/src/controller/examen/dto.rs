use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::value_object::etiqueta::Etiqueta;
use quizz_core::pregunta::domain::value_object::id::PreguntaID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;

#[derive(Deserialize)]
pub struct RegistrarExamenDTO {
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreguntaMongoDTO {
    _id: String,
    contenido: String,
    etiqueta: String,
    tipo_de_pregunta: String,
    imagen_ref: String,
    alternativas: HashMap<String, String>,
    puntaje: HashMap<String, i32>,
}

impl PreguntaMongoDTO {
    pub fn to_entity(self) -> Result<PreguntaEntity, Box<dyn std::error::Error>> {
        let id = PreguntaID::new(&self._id)?;
        let etiqueta = Etiqueta::from_str(&self.etiqueta)?;
        let tipo_de_pregunta = TipoPregunta::from_str(&self.tipo_de_pregunta)?;

        let imagen_ref = if self.imagen_ref.is_empty() {
            None
        } else {
            Some(self.imagen_ref)
        };

        let puntaje = self.puntaje
            .into_iter()
            .map(|(key, value)| (key, value as u32))
            .collect();

        Ok(PreguntaEntity {
            id,
            contenido: self.contenido,
            imagen_ref,
            etiqueta,
            tipo_de_pregunta,
            alternativas: self.alternativas,
            puntaje,
        })
    }
}
