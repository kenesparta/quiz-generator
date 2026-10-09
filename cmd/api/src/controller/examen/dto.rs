use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::value_object::etiqueta::Etiqueta;
use quizz_core::pregunta::domain::value_object::id::PreguntaID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct RegistrarExamenDTO {
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
}

/// Una pregunta tal como se guarda dentro de un examen.
#[derive(Debug, Deserialize)]
pub struct PreguntaMongoDTO {
    #[serde(rename = "_id")]
    id: String,
    contenido: String,
    etiqueta: String,
    tipo_de_pregunta: String,
    /// Opcional: las preguntas sin imagen se guardan sin el campo (o con null o "").
    #[serde(default)]
    imagen_ref: Option<String>,
    #[serde(default)]
    alternativas: HashMap<String, String>,
    /// Un puntaje negativo o fuera de rango es un dato corrupto: no se trunca con `as`.
    #[serde(default)]
    puntaje: HashMap<String, u32>,
}

impl PreguntaMongoDTO {
    pub fn into_entity(self) -> Result<PreguntaEntity, Box<dyn std::error::Error>> {
        Ok(PreguntaEntity {
            id: PreguntaID::new(&self.id)?,
            contenido: self.contenido,
            imagen_ref: self.imagen_ref.filter(|imagen| !imagen.is_empty()),
            etiqueta: Etiqueta::from_str(&self.etiqueta)?,
            tipo_de_pregunta: TipoPregunta::from_str(&self.tipo_de_pregunta)?,
            alternativas: self.alternativas,
            puntaje: self.puntaje,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::{self, Bson, doc};

    fn pregunta(imagen: Option<Bson>) -> mongodb::bson::Document {
        let mut documento = doc! {
            "_id": "e06743ff-8090-49b0-98c2-bd59db7d610f",
            "contenido": "¿Ha sido entrevistado?",
            "etiqueta": "no",
            "tipo_de_pregunta": "alternativa_unica",
            "alternativas": { "A": "Sí", "B": "No" },
            "puntaje": { "A": 1 },
        };
        if let Some(imagen) = imagen {
            documento.insert("imagen_ref", imagen);
        }
        documento
    }

    #[test]
    fn una_pregunta_sin_imagen_se_lee() {
        for imagen in [None, Some(Bson::Null), Some(Bson::String(String::new()))] {
            let dto: PreguntaMongoDTO = bson::from_document(pregunta(imagen.clone())).unwrap();
            let entidad = dto.into_entity().unwrap();
            assert_eq!(entidad.imagen_ref, None, "{imagen:?}");
        }
    }

    #[test]
    fn una_pregunta_con_imagen_la_conserva() {
        let imagen = Bson::String("data:image/png;base64,AAAA".to_string());
        let dto: PreguntaMongoDTO = bson::from_document(pregunta(Some(imagen))).unwrap();
        assert!(dto.into_entity().unwrap().imagen_ref.is_some());
    }

    #[test]
    fn un_puntaje_negativo_es_un_error_de_lectura() {
        let mut documento = pregunta(None);
        documento.insert("puntaje", doc! { "A": -1 });
        assert!(bson::from_document::<PreguntaMongoDTO>(documento).is_err());
    }
}
