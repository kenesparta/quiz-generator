use crate::controller::auth::jwt::Claims;
use crate::controller::hateoas::{Link, ListResponse, Links};
use crate::controller::respuesta::dto::{AsignacionListItemDTO, AsignacionesQueryParams};
use crate::controller::respuesta::mongo::read::ListarAsignacionesMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::listar_asignaciones::{InputData, ListarAsignaciones};
use serde_json::json;

pub struct ListarAsignacionesController;

impl ListarAsignacionesController {
    pub async fn list(
        req: HttpRequest,
        query: web::Query<AsignacionesQueryParams>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(c) => c,
            None => {
                warn!("GET /respuestas/asignaciones - claims no encontrados");
                return HttpResponse::Unauthorized()
                    .json(json!({"error": "Token no encontrado"}));
            }
        };

        let rol = claims.rol.as_deref().unwrap_or("");
        if rol != Rol::Admin.to_string() && rol != Rol::Psicologo.to_string() {
            warn!(
                "GET /respuestas/asignaciones - rol no autorizado: {}",
                rol
            );
            return HttpResponse::Forbidden().json(json!({"error": "Acceso denegado"}));
        }

        info!(
            "GET /respuestas/asignaciones (postulante_id={:?}, evaluacion_id={:?})",
            query.postulante_id, query.evaluacion_id
        );

        let listar = ListarAsignaciones::new(Box::new(ListarAsignacionesMongo::new(pool)));
        let input = InputData {
            postulante_id: query.postulante_id.clone(),
            evaluacion_id: query.evaluacion_id.clone(),
        };

        match listar.ejecutar(input).await {
            Ok(asignaciones) => {
                info!(
                    "GET /respuestas/asignaciones - {} resultados",
                    asignaciones.len()
                );

                let items: Vec<AsignacionListItemDTO> = asignaciones
                    .into_iter()
                    .map(|a| {
                        let mut links = Links::new();
                        links.insert(
                            "self".into(),
                            Link::get(format!("/respuestas/{}", a.respuesta_id)),
                        );
                        links.insert(
                            "postulante".into(),
                            Link::get(format!("/postulantes?id={}", a.postulante_id)),
                        );
                        links.insert(
                            "evaluacion".into(),
                            Link::get(format!("/evaluaciones/{}", a.evaluacion_id)),
                        );

                        let nombre_completo = [
                            a.postulante_nombre.as_str(),
                            a.postulante_primer_apellido.as_str(),
                            a.postulante_segundo_apellido.as_str(),
                        ]
                        .iter()
                        .filter(|s| !s.is_empty())
                        .copied()
                        .collect::<Vec<&str>>()
                        .join(" ");

                        AsignacionListItemDTO {
                            id: a.respuesta_id,
                            estado: a.estado,
                            fecha_tiempo_inicio: a.fecha_tiempo_inicio,
                            fecha_tiempo_fin: a.fecha_tiempo_fin,
                            evaluacion_id: a.evaluacion_id,
                            evaluacion_nombre: a.evaluacion_nombre,
                            evaluacion_descripcion: a.evaluacion_descripcion,
                            postulante_id: a.postulante_id,
                            postulante_documento: a.postulante_documento,
                            postulante_nombre: a.postulante_nombre,
                            postulante_primer_apellido: a.postulante_primer_apellido,
                            postulante_segundo_apellido: a.postulante_segundo_apellido,
                            postulante_nombre_completo: nombre_completo,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = Links::new();
                collection_links
                    .insert("self".into(), Link::get("/respuestas/asignaciones"));

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!("GET /respuestas/asignaciones - error: {}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener las asignaciones"}))
            }
        }
    }
}
