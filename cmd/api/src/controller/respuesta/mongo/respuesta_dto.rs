//! Lectura de una hoja de respuestas guardada (colección `respuesta`) y su conversión al
//! dominio.
//!
//! Las conversiones son `TryFrom`: un documento con un id, una etiqueta o un estado que no se
//! puede interpretar es un error de datos (500, con el campo en el log), nunca un `panic`.
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::pregunta::domain::value_object::etiqueta::Etiqueta;
use quizz_core::pregunta::domain::value_object::id::PreguntaID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use quizz_core::respuesta::domain::entity::evaluacion::Evaluacion;
use quizz_core::respuesta::domain::entity::examen::Examen;
use quizz_core::respuesta::domain::entity::pregunta::Pregunta;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Respuesta, Revision};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::error;

#[derive(Deserialize)]
pub struct RespuestaDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub estado: String,
    #[serde(default)]
    pub fecha_tiempo_inicio: String,
    #[serde(default)]
    pub fecha_tiempo_fin: String,
    pub postulante_id: String,
    pub evaluacion: EvaluacionDTO,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub resultado: Option<String>,
    #[serde(default)]
    pub revisado_por: Option<String>,
    #[serde(default)]
    pub fecha_revision: Option<String>,
}

#[derive(Deserialize)]
pub struct EvaluacionDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<ExamenDTO>,
}

#[derive(Deserialize)]
pub struct ExamenDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    #[serde(default)]
    pub preguntas: Vec<PreguntaDTO>,
    pub observacion: Option<String>,
}

#[derive(Deserialize)]
pub struct PreguntaDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub etiqueta: String,
    #[serde(default)]
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    #[serde(default)]
    pub respuestas: Option<Vec<String>>,
    pub puntos: Option<i64>,
}

/// Un campo guardado que no se puede interpretar.
#[derive(Debug)]
pub struct DatoNoValido {
    pub campo: &'static str,
    pub valor: String,
}

fn no_valido<E>(campo: &'static str, valor: &str) -> impl FnOnce(E) -> DatoNoValido {
    let valor = valor.to_string();
    move |_| DatoNoValido { campo, valor }
}

impl RespuestaDTO {
    /// Convierte el documento al dominio; si un campo no es válido lo registra junto al id del
    /// documento y devuelve un error de base de datos.
    pub fn a_dominio(self) -> Result<Respuesta, RespuestaError> {
        let id = self.id.clone();
        Respuesta::try_from(self).map_err(|e| {
            error!(
                "hoja de respuestas {id}: {} no valido ({:?})",
                e.campo, e.valor
            );
            RespuestaError::DatabaseError
        })
    }
}

impl TryFrom<RespuestaDTO> for Respuesta {
    type Error = DatoNoValido;

    fn try_from(respuesta: RespuestaDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: RespuestaID::new(&respuesta.id).map_err(no_valido("_id", &respuesta.id))?,
            estado: Estado::from_str(&respuesta.estado)
                .map_err(no_valido("estado", &respuesta.estado))?,
            revision: Revision::from_str(&respuesta.revision)
                .map_err(no_valido("revision", &respuesta.revision))?,
            postulante: PostulanteID::new(&respuesta.postulante_id)
                .map_err(no_valido("postulante_id", &respuesta.postulante_id))?,
            fecha_tiempo_inicio: respuesta.fecha_tiempo_inicio,
            fecha_tiempo_fin: respuesta.fecha_tiempo_fin,
            evaluacion: respuesta.evaluacion.try_into()?,
            resultado: respuesta.resultado.unwrap_or_default(),
            revisado_por: respuesta.revisado_por,
            fecha_revision: respuesta.fecha_revision,
        })
    }
}

impl TryFrom<EvaluacionDTO> for Evaluacion {
    type Error = DatoNoValido;

    fn try_from(evaluacion: EvaluacionDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: EvaluacionID::new(&evaluacion.id)
                .map_err(no_valido("evaluacion._id", &evaluacion.id))?,
            nombre: evaluacion.nombre,
            descripcion: evaluacion.descripcion,
            examenes: evaluacion
                .examenes
                .into_iter()
                .map(Examen::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<ExamenDTO> for Examen {
    type Error = DatoNoValido;

    fn try_from(examen: ExamenDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ExamenID::new(&examen.id).map_err(no_valido("examen._id", &examen.id))?,
            titulo: examen.titulo,
            descripcion: examen.descripcion,
            instrucciones: examen.instrucciones,
            observaciones: String::new(),
            preguntas: examen
                .preguntas
                .into_iter()
                .map(Pregunta::try_from)
                .collect::<Result<_, _>>()?,
            observacion: examen.observacion.unwrap_or_default(),
        })
    }
}

impl TryFrom<PreguntaDTO> for Pregunta {
    type Error = DatoNoValido;

    fn try_from(pregunta: PreguntaDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: PreguntaID::new(&pregunta.id).map_err(no_valido("pregunta._id", &pregunta.id))?,
            etiqueta: Etiqueta::from_str(&pregunta.etiqueta)
                .map_err(no_valido("etiqueta", &pregunta.etiqueta))?,
            tipo_de_pregunta: TipoPregunta::from_str(&pregunta.tipo_de_pregunta)
                .map_err(no_valido("tipo_de_pregunta", &pregunta.tipo_de_pregunta))?,
            contenido: pregunta.contenido,
            observaciones: String::new(),
            imagen_ref: pregunta.imagen_ref.unwrap_or_default(),
            alternativas: pregunta.alternativas,
            puntaje: Default::default(),
            respuestas: Some(pregunta.respuestas.unwrap_or_default()),
            puntos: pregunta.puntos.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::{self, doc};

    fn documento(etiqueta: &str) -> mongodb::bson::Document {
        doc! {
            "_id": "9abb15ef-db2c-40ca-b255-ce692cd94208",
            "estado": "en_proceso",
            "fecha_tiempo_inicio": "2026-10-05T10:00:00-05:00",
            "fecha_tiempo_fin": "",
            "postulante_id": "e17439e0-79e1-47e3-b5f9-5b54367fa290",
            "revision": "sin_iniciar",
            "evaluacion": {
                "_id": "2cf52b7a-0ee3-43a9-9b89-4a8baaa22250",
                "nombre": "Licencia",
                "descripcion": "Evaluacion",
                "examenes": [{
                    "_id": "2fcb7b0d-30e2-4853-afbf-9df79dd83ecb",
                    "titulo": "Examen",
                    "descripcion": "D",
                    "instrucciones": "I",
                    "preguntas": [{
                        "_id": "e06743ff-8090-49b0-98c2-bd59db7d610f",
                        "contenido": "¿?",
                        "tipo_de_pregunta": "alternativa_unica",
                        "etiqueta": etiqueta,
                        "alternativas": { "A": "Sí", "B": "No" },
                        "puntaje": { "A": 1 },
                        "respuestas": ["A"],
                        "puntos": 1_i64,
                    }],
                }],
            },
        }
    }

    #[test]
    fn un_documento_valido_se_convierte_y_suma_los_puntos() {
        let dto: RespuestaDTO = bson::from_document(documento("no")).unwrap();
        let respuesta = dto.a_dominio().unwrap();
        assert_eq!(respuesta.estado, Estado::EnProceso);
        assert_eq!(respuesta.evaluacion.examenes[0].puntos_obtenidos(), 1);
    }

    #[test]
    fn un_dato_no_valido_es_un_error_y_no_un_panic() {
        let dto: RespuestaDTO = bson::from_document(documento("x")).unwrap();
        let error = Respuesta::try_from(dto).err().unwrap();
        assert_eq!(error.campo, "etiqueta");
        assert_eq!(error.valor, "x");
    }
}
