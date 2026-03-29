use crate::controller::auth::jwt::Claims;
use crate::controller::hateoas::{Link, ListResponse};
use crate::controller::respuesta::dto::{
    RespuestaListItemDTO, RespuestaQueryParams, build_respuesta_list_item_links,
};
use crate::controller::respuesta::mongo::read::ListaRespuestaPostulanteMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::lista_respuesta_postulante::{
    InputData, ListaRespuestaPostulante,
};
use serde_json::json;

pub struct ListarRespuestasController;

impl ListarRespuestasController {
    pub async fn list(
        req: HttpRequest,
        query: web::Query<RespuestaQueryParams>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(c) => c,
            None => {
                warn!("GET /respuestas - claims no encontrados");
                return HttpResponse::Unauthorized().json(json!({"error": "Token no encontrado"}));
            }
        };

        let rol = claims.rol.as_deref().unwrap_or("");

        // Para postulante: siempre usar claims.sub, ignorar query param
        let postulante_id = if rol == "postulante" {
            claims.sub.clone()
        } else {
            match &query.postulante_id {
                Some(id) => id.clone(),
                None => {
                    // psicologo/admin sin filtro: por ahora requerimos el filtro
                    warn!("GET /respuestas - postulante_id requerido");
                    return HttpResponse::BadRequest()
                        .json(json!({"error": "Se requiere el parametro postulante_id"}));
                }
            }
        };

        info!("GET /respuestas?postulante_id={}", postulante_id);

        let lista_respuesta =
            ListaRespuestaPostulante::new(Box::new(ListaRespuestaPostulanteMongo::new(pool)));
        let input = InputData {
            postulante_id: postulante_id.clone(),
        };

        match lista_respuesta.ejecutar(input).await {
            Ok(respuestas) => {
                info!(
                    "GET /respuestas?postulante_id={} - {} resultados",
                    postulante_id,
                    respuestas.len()
                );

                let items: Vec<RespuestaListItemDTO> = respuestas
                    .into_iter()
                    .map(|r| {
                        let links =
                            build_respuesta_list_item_links(&r.respuesta_id, &r.estado, rol);
                        RespuestaListItemDTO {
                            id: r.respuesta_id,
                            nombre_evaluacion: r.nombre_evaluacion,
                            descripcion_evaluacion: r.descripcion_evaluacion,
                            estado: r.estado,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = crate::controller::hateoas::Links::new();
                collection_links.insert(
                    "self".into(),
                    Link::get(format!("/respuestas?postulante_id={}", postulante_id)),
                );

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!(
                    "GET /respuestas?postulante_id={} - error: {}",
                    postulante_id, e
                );
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener respuestas"}))
            }
        }
    }
}
