use crate::controller::hateoas::{Link, Links};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_core::respuesta::domain::entity::respuesta::Estado;
use quizz_core::respuesta::use_case::respuesta_postulante::OutputData;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// --- Request DTOs ---

#[derive(Deserialize)]
pub struct CrearRespuestaDTO {
    pub postulante_id: String,
}

#[derive(Deserialize)]
pub struct TransicionEstadoDTO {
    pub accion: AccionTransicion,
}

/// Una acción desconocida la rechaza el extractor JSON con 400.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccionTransicion {
    Empezar,
    Finalizar,
}

#[derive(Deserialize)]
pub struct ContestacionDTO {
    pub respuestas: Vec<String>,
}

#[derive(Deserialize)]
pub struct RespuestaQueryParams {
    pub postulante_id: Option<String>,
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
    /// Ordenadas por clave (A, B, C...), no en el orden aleatorio de un HashMap.
    pub alternativas: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respuestas: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puntos: Option<i64>,
    #[serde(rename = "_links")]
    pub links: Links,
}

// --- Vistas ---

impl RespuestaDetailDTO {
    /// La hoja tal como la puede ver `rol`.
    ///
    /// El postulante no ve puntos, totales, observaciones ni resultado: con los puntos de cada
    /// intento podía volver a contestar hasta deducir la clave de corrección (SEC-09). Tampoco
    /// ve las preguntas antes de empezar, para que el tiempo registrado refleje el examen
    /// (R-092). El personal lo ve todo, más el enlace al postulante dueño de la hoja.
    pub fn para(rol: Rol, r: OutputData) -> Self {
        let personal = rol != Rol::Postulante;
        let ocultar_preguntas = !personal && r.estado == Estado::Creado;

        let mut links = build_respuesta_links(&r.id, r.estado, rol);
        if personal {
            links.insert(
                "postulante".into(),
                Link::get(format!("/postulantes?id={}", r.postulante_id)),
            );
        }

        let examenes = r
            .evaluacion
            .examenes
            .into_iter()
            .map(|ex| {
                let preguntas = if ocultar_preguntas {
                    Vec::new()
                } else {
                    ex.preguntas
                        .into_iter()
                        .map(|p| PreguntaResponseDTO {
                            links: build_pregunta_links(&r.id, &ex.id, &p.id, r.estado, rol),
                            id: p.id,
                            contenido: p.contenido,
                            tipo_de_pregunta: p.tipo_de_pregunta,
                            etiqueta: String::new(),
                            imagen_ref: Some(p.imagen_ref).filter(|imagen| !imagen.is_empty()),
                            alternativas: p.alternativas.into_iter().collect(),
                            respuestas: p.respuestas,
                            puntos: personal.then_some(p.puntos),
                        })
                        .collect()
                };
                ExamenResponseDTO {
                    id: ex.id,
                    titulo: ex.titulo,
                    descripcion: ex.descripcion,
                    instrucciones: ex.instrucciones,
                    preguntas,
                    puntos_obtenidos: personal.then_some(ex.puntos_obtenidos),
                    observacion: personal.then_some(ex.observacion),
                }
            })
            .collect();

        Self {
            fecha_tiempo_transcurrido: r.fecha_tiempo_transcurrido,
            fecha_tiempo_inicio: r.fecha_tiempo_inicio,
            fecha_tiempo_fin: r.fecha_tiempo_fin,
            estado: r.estado.to_string(),
            evaluacion: EvaluacionResponseDTO {
                id: r.evaluacion.id,
                nombre: r.evaluacion.nombre,
                descripcion: r.evaluacion.descripcion,
                examenes,
            },
            revision: r.revision,
            resultado: Some(r.resultado).filter(|resultado| personal && !resultado.is_empty()),
            id: r.id,
            links,
        }
    }
}

// --- Link builders ---

/// Enlaces de una hoja de respuestas: solo ofrece las transiciones válidas para su estado y el
/// rol de quien la mira.
pub fn build_respuesta_links(respuesta_id: &str, estado: Estado, rol: Rol) -> Links {
    let mut links = Links::new();
    links.insert(
        "self".into(),
        Link::get(format!("/respuestas/{respuesta_id}")),
    );

    match (estado, rol) {
        (Estado::Creado, Rol::Postulante) => {
            links.insert(
                "empezar".into(),
                Link::patch(format!("/respuestas/{respuesta_id}/estado")),
            );
        }
        (Estado::EnProceso, Rol::Postulante) => {
            links.insert(
                "finalizar".into(),
                Link::patch(format!("/respuestas/{respuesta_id}/estado")),
            );
        }
        (Estado::Finalizado, Rol::Psicologo | Rol::Admin) => {
            links.insert(
                "revision".into(),
                Link::get(format!("/revisiones/{respuesta_id}")),
            );
            links.insert(
                "revisar".into(),
                Link::post(format!("/revisiones/{respuesta_id}")),
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
    estado: Estado,
    rol: Rol,
) -> Links {
    let mut links = Links::new();

    if estado == Estado::EnProceso && rol == Rol::Postulante {
        links.insert(
            "contestar".into(),
            Link::post(format!(
                "/respuestas/{respuesta_id}/examenes/{examen_id}/preguntas/{pregunta_id}/contestaciones"
            )),
        );
    }

    links
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizz_core::respuesta::use_case::respuesta_postulante::{
        OutputEvaluacion, OutputExamen, OutputPregunta,
    };
    use std::collections::HashMap;

    fn hoja(estado: Estado) -> OutputData {
        OutputData {
            id: "hoja".to_string(),
            postulante_id: "dueno".to_string(),
            fecha_tiempo_inicio: String::new(),
            fecha_tiempo_transcurrido: None,
            fecha_tiempo_fin: String::new(),
            estado,
            revision: "finalizada".to_string(),
            resultado: "apto".to_string(),
            evaluacion: OutputEvaluacion {
                id: "evaluacion".to_string(),
                nombre: "N".to_string(),
                descripcion: "D".to_string(),
                examenes: vec![OutputExamen {
                    id: "examen".to_string(),
                    titulo: "T".to_string(),
                    descripcion: "D".to_string(),
                    instrucciones: "I".to_string(),
                    puntos_obtenidos: 4,
                    observacion: "nota privada".to_string(),
                    preguntas: vec![OutputPregunta {
                        id: "pregunta".to_string(),
                        contenido: "¿?".to_string(),
                        tipo_de_pregunta: "alternativa_unica".to_string(),
                        imagen_ref: String::new(),
                        alternativas: HashMap::from([
                            ("B".to_string(), "No".to_string()),
                            ("A".to_string(), "Sí".to_string()),
                        ]),
                        respuestas: Some(vec!["A".to_string()]),
                        puntos: 4,
                    }],
                }],
            },
        }
    }

    fn json(rol: Rol, estado: Estado) -> String {
        serde_json::to_string(&RespuestaDetailDTO::para(rol, hoja(estado))).unwrap()
    }

    #[test]
    fn el_postulante_no_ve_puntos_ni_notas_ni_resultado() {
        let vista = json(Rol::Postulante, Estado::EnProceso);
        for campo in ["\"puntos\"", "puntos_obtenidos", "observacion", "resultado"] {
            assert!(!vista.contains(campo), "{campo} en {vista}");
        }
        assert!(vista.contains("contestar"), "{vista}");
    }

    #[test]
    fn el_personal_ve_puntos_notas_resultado_y_al_postulante() {
        for rol in [Rol::Psicologo, Rol::Admin] {
            let vista = json(rol, Estado::Finalizado);
            for campo in [
                "\"puntos\":4",
                "\"puntos_obtenidos\":4",
                "nota privada",
                "apto",
            ] {
                assert!(vista.contains(campo), "{rol}: {campo} en {vista}");
            }
            assert!(vista.contains("/postulantes?id=dueno"), "{vista}");
        }
    }

    #[test]
    fn el_postulante_no_ve_las_preguntas_antes_de_empezar() {
        let vista = RespuestaDetailDTO::para(Rol::Postulante, hoja(Estado::Creado));
        assert!(vista.evaluacion.examenes[0].preguntas.is_empty());
        assert!(vista.links.contains_key("empezar"));

        let personal = RespuestaDetailDTO::para(Rol::Psicologo, hoja(Estado::Creado));
        assert_eq!(personal.evaluacion.examenes[0].preguntas.len(), 1);
    }

    #[test]
    fn las_alternativas_salen_en_orden() {
        let vista = json(Rol::Postulante, Estado::EnProceso);
        let a = vista.find("\"A\":").unwrap();
        let b = vista.find("\"B\":").unwrap();
        assert!(a < b, "{vista}");
    }
}
