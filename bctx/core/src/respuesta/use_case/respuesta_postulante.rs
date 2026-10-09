use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::entity::examen::Examen;
use crate::respuesta::domain::entity::pregunta::Pregunta;
use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioRespuestaLectura;
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InputData {
    pub respuesta_id: String,
    /// `Some(dueño)` en la lectura de un postulante: solo obtiene su propia hoja. `None` en la
    /// lectura del personal, que puede ver cualquiera.
    pub postulante_id: Option<String>,
    pub ahora: DateTime<FixedOffset>,
}

pub struct OutputData {
    pub id: String,
    pub postulante_id: String,
    pub fecha_tiempo_inicio: String,
    /// Segundos dedicados (hasta el fin, o hasta ahora si sigue en proceso).
    pub fecha_tiempo_transcurrido: Option<i64>,
    pub fecha_tiempo_fin: String,
    pub estado: Estado,
    pub evaluacion: OutputEvaluacion,
    pub revision: String,
    pub resultado: String,
}

pub struct OutputEvaluacion {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<OutputExamen>,
}

impl From<Evaluacion> for OutputEvaluacion {
    fn from(evaluacion: Evaluacion) -> Self {
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

pub struct OutputExamen {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<OutputPregunta>,
    pub puntos_obtenidos: i64,
    pub observacion: String,
}

impl From<Examen> for OutputExamen {
    fn from(examen: Examen) -> Self {
        let puntos_obtenidos = examen.puntos_obtenidos();
        Self {
            id: examen.id.to_string(),
            titulo: examen.titulo,
            descripcion: examen.descripcion,
            instrucciones: examen.instrucciones,
            preguntas: examen
                .preguntas
                .into_iter()
                .map(|pregunta| pregunta.into())
                .collect(),
            puntos_obtenidos,
            observacion: examen.observacion,
        }
    }
}

pub struct OutputPregunta {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,
    pub respuestas: Option<Vec<String>>,
    pub puntos: i64,
}

impl From<Pregunta> for OutputPregunta {
    fn from(pregunta: Pregunta) -> Self {
        Self {
            id: pregunta.id.to_string(),
            contenido: pregunta.contenido,
            tipo_de_pregunta: pregunta.tipo_de_pregunta.to_string(),
            imagen_ref: pregunta.imagen_ref.to_string(),
            alternativas: pregunta.alternativas,
            respuestas: pregunta.respuestas,
            puntos: pregunta.puntos,
        }
    }
}

/// Una hoja de respuestas con su evaluación, para el postulante dueño o para el personal.
pub struct ObtenerRespuesta<R> {
    repositorio: R,
}

impl<R: RepositorioRespuestaLectura> ObtenerRespuesta<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, input: InputData) -> Result<OutputData, RespuestaError> {
        let respuesta_id = RespuestaID::new(&input.respuesta_id)?;
        let postulante_id = input
            .postulante_id
            .as_deref()
            .map(PostulanteID::new)
            .transpose()?;

        let respuesta = self
            .repositorio
            .obtener(&respuesta_id, postulante_id.as_ref())
            .await?;

        Ok(OutputData {
            fecha_tiempo_transcurrido: respuesta.segundos_transcurridos(input.ahora),
            id: respuesta.id.to_string(),
            postulante_id: respuesta.postulante.to_string(),
            fecha_tiempo_inicio: respuesta.fecha_tiempo_inicio,
            fecha_tiempo_fin: respuesta.fecha_tiempo_fin,
            estado: respuesta.estado,
            evaluacion: respuesta.evaluacion.into(),
            revision: respuesta.revision.to_string(),
            resultado: respuesta.resultado,
        })
    }
}
