use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::entity::examen::Examen;
use crate::respuesta::domain::entity::pregunta::Pregunta;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioObtenerRevisionPorId;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::collections::HashMap;

pub struct InputData {
    pub revision_id: String,
}

pub struct OutputData {
    pub id: String,
    pub postulante_id: String,
    pub resultado: String,
    pub revision: String,
    pub evaluacion: OutputEvaluacion,
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
            observacion: examen.observacion,
        }
    }
}

pub struct OutputPregunta {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
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
            alternativas: pregunta.alternativas,
            respuestas: pregunta.respuestas,
            puntos: pregunta.puntos,
        }
    }
}

pub struct ObtenerRevisionPorId<RepoErr> {
    repo: Box<dyn RepositorioObtenerRevisionPorId<RepoErr>>,
}

impl<RepoErr> ObtenerRevisionPorId<RepoErr> {
    pub fn new(repo: Box<dyn RepositorioObtenerRevisionPorId<RepoErr>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, RespuestaError> for ObtenerRevisionPorId<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, input: InputData) -> Result<OutputData, RespuestaError> {
        let respuesta = self
            .repo
            .obtener_revision_por_id(input.revision_id)
            .await?;

        Ok(OutputData {
            id: respuesta.id.to_string(),
            postulante_id: respuesta.postulante.to_string(),
            resultado: respuesta.resultado,
            revision: respuesta.revision.to_string(),
            evaluacion: respuesta.evaluacion.into(),
        })
    }
}
