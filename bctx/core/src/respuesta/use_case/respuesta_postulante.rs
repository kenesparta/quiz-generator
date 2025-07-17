use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::pregunta::Pregunta;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioRespuestaLectura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::collections::HashMap;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::entity::examen::Examen;

#[derive(Debug, Clone)]
pub struct InputData {
    pub postulante_id: String,
}

pub struct OutputData {
    pub id: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion: OutputEvaluacion,
    pub postulante: OutputPostulante,
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
        }
    }
}

pub struct OutputPregunta {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,
    pub respuestas: Vec<String>,
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
        }
    }
}

pub struct OutputPostulante {
    pub id: String,
    pub documento: String,
    pub nombre_completo: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}

impl From<Postulante> for OutputPostulante {
    fn from(postulante: Postulante) -> Self {
        Self {
            id: postulante.id.to_string(),
            documento: postulante.documento.to_string(),
            nombre_completo: postulante.nombre_completo.nombre_completo(),
            fecha_nacimiento: postulante.fecha_nacimiento.to_string(),
            grado_instruccion: postulante.grado_instruccion.to_string(),
            genero: postulante.genero.to_string(),
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
            .obtener_por_postulante(postulante_id)
            .await?;

        Ok(OutputData {
            id: respuestas.id.to_string(),
            fecha_tiempo_inicio: respuestas.fecha_tiempo_inicio.to_string(),
            fecha_tiempo_fin: respuestas.fecha_tiempo_fin.to_string(),
            evaluacion: respuestas.evaluacion.into(),
            postulante: respuestas.postulante_details.into(),
        })
    }
}
