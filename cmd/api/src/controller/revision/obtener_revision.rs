use crate::controller::auth::jwt::Claims;
use crate::controller::hateoas::{Link, Links};
use crate::controller::psicologo::mongo::read::PsicologoReadMongo;
use crate::controller::revision::dto::{
    RevisionDetalleDTO, RevisionEvaluacionDTO, RevisionExamenDTO, RevisionPreguntaDTO,
    RevisionPsicologoDTO,
};
use crate::controller::revision::mongo::read::RevisionReadMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::psicologo::provider::repositorio::RepositorioPsicologoLectura;
use quizz_core::respuesta::use_case::obtener_revision::{InputData, ObtenerRevisionPorId};
use serde_json::json;

pub struct ObtenerRevisionController;

impl ObtenerRevisionController {
    pub async fn get(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let revision_id = match req.match_info().get("revision_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la revision"}));
            }
        };

        info!("GET /revisiones/{}", revision_id);

        let obtener_revision =
            ObtenerRevisionPorId::new(Box::new(RevisionReadMongo::new(pool.clone())));

        match obtener_revision
            .ejecutar(InputData {
                revision_id: revision_id.clone(),
            })
            .await
        {
            Ok(output) => {
                info!("GET /revisiones/{} - encontrado", revision_id);

                // Obtener datos del psicólogo si el solicitante es psicólogo
                let claims = req.extensions().get::<Claims>().cloned();
                let psicologo_dto = if let Some(claims) =
                    claims.filter(|c| c.rol.as_deref() == Some(&Rol::Psicologo.to_string()))
                {
                    let psicologo_read = PsicologoReadMongo::new(pool);
                    match psicologo_read.obtener_psicologo_por_id(claims.sub).await {
                        Ok(info) => Some(RevisionPsicologoDTO {
                            nombre_completo: format!(
                                "{} {} {}",
                                info.nombre, info.primer_apellido, info.segundo_apellido
                            ),
                            colegiatura: info.colegiatura,
                        }),
                        Err(e) => {
                            warn!(
                                "GET /revisiones/{} - no se pudo obtener psicologo: {:?}",
                                revision_id, e
                            );
                            None
                        }
                    }
                } else {
                    None
                };

                let mut links = Links::new();
                links.insert(
                    "self".into(),
                    Link::get(format!("/revisiones/{}", output.id)),
                );
                links.insert(
                    "postulante".into(),
                    Link::get(format!("/postulantes?id={}", output.postulante_id)),
                );

                HttpResponse::Ok().json(RevisionDetalleDTO {
                    id: output.id,
                    postulante_id: output.postulante_id,
                    resultado: output.resultado,
                    revision: output.revision,
                    fecha_tiempo_inicio: output.fecha_tiempo_inicio,
                    fecha_tiempo_fin: output.fecha_tiempo_fin,
                    psicologo: psicologo_dto,
                    evaluacion: RevisionEvaluacionDTO {
                        id: output.evaluacion.id,
                        nombre: output.evaluacion.nombre,
                        descripcion: output.evaluacion.descripcion,
                        examenes: output
                            .evaluacion
                            .examenes
                            .into_iter()
                            .map(|ex| RevisionExamenDTO {
                                id: ex.id,
                                titulo: ex.titulo,
                                descripcion: ex.descripcion,
                                instrucciones: ex.instrucciones,
                                preguntas: ex
                                    .preguntas
                                    .into_iter()
                                    .map(|p| RevisionPreguntaDTO {
                                        id: p.id,
                                        contenido: p.contenido,
                                        tipo_de_pregunta: p.tipo_de_pregunta,
                                        imagen_ref: if p.imagen_ref.is_empty() {
                                            None
                                        } else {
                                            Some(p.imagen_ref)
                                        },
                                        alternativas: p.alternativas,
                                        respuestas: p.respuestas,
                                        puntos: p.puntos,
                                    })
                                    .collect(),
                                puntos_obtenidos: ex.puntos_obtenidos,
                                observacion: ex.observacion,
                            })
                            .collect(),
                    },
                    links,
                })
            }
            Err(err) => {
                warn!("GET /revisiones/{} - error: {:?}", revision_id, err);
                match err {
                    quizz_core::respuesta::domain::error::respuesta::RespuestaError::RespuestaNoEncontrada => {
                        HttpResponse::NotFound()
                            .json(json!({"error": "Revision no encontrada o la evaluacion no ha sido finalizada"}))
                    }
                    _ => {
                        error!("GET /revisiones/{} - error inesperado: {:?}", revision_id, err);
                        HttpResponse::InternalServerError()
                            .json(json!({"error": "Error al obtener la revision"}))
                    }
                }
            }
        }
    }
}
