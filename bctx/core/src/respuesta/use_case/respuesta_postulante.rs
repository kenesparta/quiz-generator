use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::entity::examen::Examen;
use crate::respuesta::domain::entity::pregunta::Pregunta;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioRespuestaLectura;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use quizz_common::use_case::CasoDeUso;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InputData {
    pub postulante_id: String,
    pub respuesta_id: String,
}

pub struct OutputData {
    pub id: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_transcurrido: i64,
    pub fecha_tiempo_fin: String,
    pub evaluacion: OutputEvaluacion,
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
    pub observacion: Option<String>,
}

impl From<Examen> for OutputExamen {
    fn from(examen: Examen) -> Self {
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
            puntos_obtenidos: examen.puntos_obtenidos,
            observacion: Option::from(examen.observacion),
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

pub struct RespuestaPorPostulante<RepoErr> {
    repositorio: Box<dyn RepositorioRespuestaLectura<RepoErr>>,
}

impl<RepoErr> RespuestaPorPostulante<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioRespuestaLectura<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, RespuestaError> for RespuestaPorPostulante<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, input: InputData) -> Result<OutputData, RespuestaError> {
        let postulante_id = PostulanteID::new(&input.postulante_id)?;
        let respuestas = self
            .repositorio
            .obtener_por_postulante(input.respuesta_id, postulante_id)
            .await?;

        let fecha_inicio_str = respuestas.fecha_tiempo_inicio.to_string();

        // todo: pasar esta logica al dominio
        // todo: REVISAR ESTO!!!!
        let fecha_tiempo_transcurrido =
            if let Ok(fecha_inicio) = DateTime::parse_from_rfc3339(&fecha_inicio_str) {
                let now = Utc::now();
                let duration = now.signed_duration_since(fecha_inicio.with_timezone(&Utc));
                duration.num_seconds()
            } else {
                0
            };

        Ok(OutputData {
            id: respuestas.id.to_string(),
            fecha_tiempo_inicio: fecha_inicio_str,
            fecha_tiempo_transcurrido,
            fecha_tiempo_fin: respuestas.fecha_tiempo_fin.to_string(),
            evaluacion: respuestas.evaluacion.into(),
            resultado: respuestas.resultado,
        })
    }
}
