use crate::controller::hateoas::{Link, Links};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Request DTOs ---

#[derive(Deserialize)]
pub struct CrearRespuestaDTO {
    pub postulante_id: String,
}

#[derive(Deserialize)]
pub struct TransicionEstadoDTO {
    pub accion: String,
}

#[derive(Deserialize)]
pub struct ContestacionDTO {
    pub respuestas: Vec<String>,
}

#[derive(Deserialize)]
pub struct RespuestaQueryParams {
    pub postulante_id: Option<String>,
    #[allow(dead_code)]
    pub estado: Option<String>,
}

#[derive(Deserialize)]
pub struct AsignacionesQueryParams {
    pub postulante_id: Option<String>,
    pub evaluacion_id: Option<String>,
}

// --- Response DTOs ---

#[derive(Serialize)]
pub struct RespuestaCreatedDTO {
    pub id: String,
    pub estado: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct RespuestaListItemDTO {
    pub id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct AsignacionListItemDTO {
    pub id: String,
    pub estado: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion_id: String,
    pub evaluacion_nombre: String,
    pub evaluacion_descripcion: String,
    pub postulante_id: String,
    pub postulante_documento: String,
    pub postulante_nombre: String,
    pub postulante_primer_apellido: String,
    pub postulante_segundo_apellido: String,
    pub postulante_nombre_completo: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct RespuestaDetailDTO {
    pub id: String,
    pub fecha_tiempo_inicio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_tiempo_transcurrido: Option<i64>,
    pub fecha_tiempo_fin: String,
    pub estado: String,
    pub evaluacion: EvaluacionResponseDTO,
    pub revision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resultado: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
pub struct EvaluacionResponseDTO {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<ExamenResponseDTO>,
}

#[derive(Serialize)]
pub struct ExamenResponseDTO {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<PreguntaResponseDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puntos_obtenidos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observacion: Option<String>,
}

#[derive(Serialize)]
pub struct PreguntaResponseDTO {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub etiqueta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respuestas: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puntos: Option<i64>,
    #[serde(rename = "_links")]
    pub links: Links,
}

// --- Mongo DTOs (internal) ---

#[derive(Serialize, Deserialize)]
pub struct EvaluacionMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<ExamenMongoDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct ExamenMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<PreguntaMongoDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct PreguntaMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub contenido: String,
    pub etiqueta: String,
    pub tipo_de_pregunta: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    pub puntaje: HashMap<String, u32>,
    pub respuestas: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct RespuestaMongoDTO {
    #[serde(rename = "_id")]
    pub id: String,
    pub evaluacion: EvaluacionMongoDTO,
    pub postulante_id: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub estado: String,
    pub revision: String,
}

// --- Link builders ---

pub fn build_respuesta_links(
    respuesta_id: &str,
    postulante_id: &str,
    estado: &str,
    rol: &str,
) -> Links {
    let mut links = Links::new();

    links.insert(
        "self".into(),
        Link::get(format!("/respuestas/{}", respuesta_id)),
    );

    match estado {
        "Creado" if rol == Rol::Postulante.to_string() => {
            links.insert(
                "empezar".into(),
                Link::patch(format!("/respuestas/{}/estado", respuesta_id)),
            );
        }
        "EnProceso" if rol == Rol::Postulante.to_string() => {
            links.insert(
                "finalizar".into(),
                Link::patch(format!("/respuestas/{}/estado", respuesta_id)),
            );
        }
        "Finalizado" if rol == Rol::Psicologo.to_string() || rol == Rol::Admin.to_string() => {
            links.insert(
                "revision".into(),
                Link::get(format!("/revisiones/{}", respuesta_id)),
            );
            links.insert(
                "revisar".into(),
                Link::post(format!("/revisiones/{}", respuesta_id)),
            );
        }
        _ => {}
    }

    if rol == Rol::Psicologo.to_string() || rol == Rol::Admin.to_string() {
        links.insert(
            "postulante".into(),
            Link::get(format!("/postulantes?id={}", postulante_id)),
        );
    }

    links
}

pub fn build_respuesta_list_item_links(respuesta_id: &str, estado: &str, rol: &str) -> Links {
    let mut links = Links::new();

    links.insert(
        "self".into(),
        Link::get(format!("/respuestas/{}", respuesta_id)),
    );

    match estado {
        "Creado" if rol == Rol::Postulante.to_string() => {
            links.insert(
                "empezar".into(),
                Link::patch(format!("/respuestas/{}/estado", respuesta_id)),
            );
        }
        "EnProceso" if rol == Rol::Postulante.to_string() => {
            links.insert(
                "finalizar".into(),
                Link::patch(format!("/respuestas/{}/estado", respuesta_id)),
            );
        }
        "Finalizado" if rol == Rol::Psicologo.to_string() || rol == Rol::Admin.to_string() => {
            links.insert(
                "revisar".into(),
                Link::post(format!("/revisiones/{}", respuesta_id)),
            );
        }
        _ => {}
    }

    links
}

pub fn build_pregunta_links(
    respuesta_id: &str,
    examen_id: &str,
    pregunta_id: &str,
    estado: &str,
    rol: &str,
) -> Links {
    let mut links = Links::new();

    if estado == "EnProceso" && rol == Rol::Postulante.to_string() {
        links.insert(
            "contestar".into(),
            Link::post(format!(
                "/respuestas/{}/examenes/{}/preguntas/{}/contestaciones",
                respuesta_id, examen_id, pregunta_id
            )),
        );
    }

    links
}
