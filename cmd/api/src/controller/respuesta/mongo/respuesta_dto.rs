use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::pregunta::domain::value_object::etiqueta::Etiqueta;
use quizz_core::pregunta::domain::value_object::id::PreguntaID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use quizz_core::respuesta::domain::entity::evaluacion::Evaluacion;
use quizz_core::respuesta::domain::entity::examen::Examen;
use quizz_core::respuesta::domain::entity::pregunta::Pregunta;
use quizz_core::respuesta::domain::entity::respuesta::{Respuesta, Revision};
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use quizz_core::respuesta::use_case::respuesta_postulante::{
    OutputData, OutputEvaluacion, OutputExamen, OutputPregunta,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct RespuestaDTO {
    #[serde(rename = "_id")]
    pub id: String,

    pub fecha_tiempo_inicio: String,

    #[serde(skip_deserializing)]
    pub fecha_tiempo_transcurrido: i64,

    pub fecha_tiempo_fin: String,
    pub postulante_id: String,
    pub evaluacion: EvaluacionDTO,
    pub revision: String,
}

impl From<RespuestaDTO> for Respuesta {
    fn from(respuesta: RespuestaDTO) -> Self {
        let revision =
            Revision::from_str(respuesta.revision.as_str()).unwrap_or_else(|_| Revision::Default);
        Self {
            id: RespuestaID::new(respuesta.id.as_str()).unwrap(),
            fecha_tiempo_inicio: respuesta.fecha_tiempo_inicio,
            fecha_tiempo_fin: respuesta.fecha_tiempo_fin,
            evaluacion: respuesta.evaluacion.into(),
            postulante: PostulanteID::new(respuesta.postulante_id.as_str()).unwrap(),
            revision,
        }
    }
}

impl From<OutputData> for RespuestaDTO {
    fn from(respuesta: OutputData) -> Self {
        Self {
            id: respuesta.id,
            fecha_tiempo_inicio: respuesta.fecha_tiempo_inicio,
            fecha_tiempo_transcurrido: respuesta.fecha_tiempo_transcurrido,
            fecha_tiempo_fin: respuesta.fecha_tiempo_fin,
            postulante_id: "".to_owned(),
            evaluacion: respuesta.evaluacion.into(),
            revision: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EvaluacionDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<ExamenDTO>,
}

impl From<EvaluacionDTO> for Evaluacion {
    fn from(evaluacion: EvaluacionDTO) -> Self {
        Self {
            id: EvaluacionID::new(evaluacion.id.as_str()).unwrap(),
            nombre: evaluacion.nombre,
            descripcion: evaluacion.descripcion,
            examenes: evaluacion
                .examenes
                .into_iter()
                .map(|examen| examen.into())
                .collect(),
        }
    }
}

impl From<OutputEvaluacion> for EvaluacionDTO {
    fn from(evaluacion: OutputEvaluacion) -> Self {
        Self {
            id: evaluacion.id.to_string(),
            nombre: evaluacion.nombre,
            descripcion: evaluacion.descripcion,
            examenes: evaluacion
                .examenes
                .into_iter()
                .map(|examen| examen.into())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExamenDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<PreguntaDTO>,
    pub puntos_obtenidos: Option<i64>,
}

impl From<ExamenDTO> for Examen {
    fn from(examen: ExamenDTO) -> Self {
        Self {
            id: ExamenID::new(examen.id.as_str()).unwrap(),
            titulo: examen.titulo,
            descripcion: examen.descripcion,
            instrucciones: examen.instrucciones,
            observaciones: "".to_owned(),
            preguntas: examen
                .preguntas
                .into_iter()
                .map(|pregunta| pregunta.into())
                .collect(),
            puntos_obtenidos: examen.puntos_obtenidos.unwrap_or_default(),
        }
    }
}

impl From<OutputExamen> for ExamenDTO {
    fn from(examen: OutputExamen) -> Self {
        Self {
            id: examen.id,
            titulo: examen.titulo,
            descripcion: examen.descripcion,
            instrucciones: examen.instrucciones,
            preguntas: examen
                .preguntas
                .into_iter()
                .map(|pregunta| pregunta.into())
                .collect(),
            puntos_obtenidos: Option::from(examen.puntos_obtenidos),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PreguntaDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub etiqueta: String,
    // pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,
    #[serde(default)]
    pub respuestas: Option<Vec<String>>,
    pub puntos: Option<i64>,
}

impl From<PreguntaDTO> for Pregunta {
    fn from(pregunta: PreguntaDTO) -> Self {
        Self {
            id: PreguntaID::new(pregunta.id.as_str()).unwrap(),
            contenido: pregunta.contenido,
            observaciones: "".to_string(),
            etiqueta: Etiqueta::from_str(&*pregunta.etiqueta).unwrap(),
            tipo_de_pregunta: TipoPregunta::from_str(&*pregunta.tipo_de_pregunta).unwrap(),
            imagen_ref: "".to_string(),
            alternativas: pregunta.alternativas,
            puntaje: Default::default(),
            respuestas: Option::from(pregunta.respuestas.unwrap_or_default()),
            puntos: pregunta.puntos.unwrap_or_default(),
        }
    }
}

impl From<OutputPregunta> for PreguntaDTO {
    fn from(pregunta: OutputPregunta) -> Self {
        Self {
            id: pregunta.id,
            contenido: pregunta.contenido,
            tipo_de_pregunta: pregunta.tipo_de_pregunta,
            etiqueta: "".to_owned(),
            // imagen_ref: pregunta.imagen_ref.to_string(),
            alternativas: pregunta.alternativas,
            respuestas: pregunta.respuestas,
            puntos: Option::from(pregunta.puntos),
        }
    }
}

#[derive(Serialize)]
pub struct RespuestaRevisionDTO {
    pub revision_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado_revision: String,
    pub postulante_id: String,
}
