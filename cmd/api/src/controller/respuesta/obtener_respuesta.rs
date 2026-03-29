use crate::controller::auth::jwt::Claims;
use crate::controller::respuesta::dto::{
    EvaluacionResponseDTO, ExamenResponseDTO, PreguntaResponseDTO, RespuestaDetailDTO,
    build_pregunta_links, build_respuesta_links,
};
use crate::controller::respuesta::mongo::read::RespuestaPorPostulanteMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_postulante::{InputData, RespuestaPorPostulante};
use serde_json::json;

pub struct ObtenerRespuestaController;

impl ObtenerRespuestaController {
    pub async fn get(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la respuesta"}));
            }
        };

        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(c) => c,
            None => {
                return HttpResponse::Unauthorized().json(json!({"error": "Token no encontrado"}));
            }
        };

        let rol = claims.rol.as_deref().unwrap_or("");

        // Para postulante: usar claims.sub como postulante_id
        let postulante_id = if rol == "postulante" {
            claims.sub.clone()
        } else {
            // psicologo/admin: usar el claims.sub (la query ya valida por postulante en el repo)
            // TODO: agregar obtener_por_id sin postulante_id para admin/psicologo
            claims.sub.clone()
        };

        info!("GET /respuestas/{}", respuesta_id);

        let resp_post =
            RespuestaPorPostulante::new(Box::new(RespuestaPorPostulanteMongo::new(pool)));
        let input = InputData {
            postulante_id: postulante_id.clone(),
            respuesta_id: respuesta_id.clone(),
        };

        match resp_post.ejecutar(input).await {
            Ok(r) => {
                info!("GET /respuestas/{} - encontrado", respuesta_id);

                let estado = &r.estado;
                let links = build_respuesta_links(&respuesta_id, &postulante_id, estado, rol);

                let evaluacion = EvaluacionResponseDTO {
                    id: r.evaluacion.id.to_string(),
                    nombre: r.evaluacion.nombre,
                    descripcion: r.evaluacion.descripcion,
                    examenes: r
                        .evaluacion
                        .examenes
                        .into_iter()
                        .map(|ex| ExamenResponseDTO {
                            id: ex.id.clone(),
                            titulo: ex.titulo,
                            descripcion: ex.descripcion,
                            instrucciones: ex.instrucciones,
                            preguntas: ex
                                .preguntas
                                .into_iter()
                                .map(|p| {
                                    let pregunta_links = build_pregunta_links(
                                        &respuesta_id,
                                        &ex.id,
                                        &p.id,
                                        estado,
                                        rol,
                                    );
                                    PreguntaResponseDTO {
                                        id: p.id,
                                        contenido: p.contenido,
                                        tipo_de_pregunta: p.tipo_de_pregunta,
                                        etiqueta: String::new(),
                                        alternativas: p.alternativas,
                                        respuestas: p.respuestas,
                                        puntos: Option::from(p.puntos),
                                        links: pregunta_links,
                                    }
                                })
                                .collect(),
                            puntos_obtenidos: Option::from(ex.puntos_obtenidos),
                            observacion: ex.observacion,
                        })
                        .collect(),
                };

                let fecha_tiempo_transcurrido = if r.fecha_tiempo_transcurrido > 0 {
                    Some(r.fecha_tiempo_transcurrido)
                } else {
                    None
                };

                HttpResponse::Ok().json(RespuestaDetailDTO {
                    id: r.id,
                    fecha_tiempo_inicio: r.fecha_tiempo_inicio,
                    fecha_tiempo_transcurrido,
                    fecha_tiempo_fin: r.fecha_tiempo_fin,
                    estado: estado.to_string(),
                    evaluacion,
                    revision: r.revision,
                    resultado: if r.resultado.is_empty() {
                        None
                    } else {
                        Some(r.resultado)
                    },
                    links,
                })
            }
            Err(e) => {
                warn!("GET /respuestas/{} - error: {}", respuesta_id, e);
                HttpResponse::NotFound().json(json!({"error": "Respuesta no encontrada"}))
            }
        }
    }
}
