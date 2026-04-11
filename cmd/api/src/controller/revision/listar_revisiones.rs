use crate::controller::hateoas::{Link, Links, ListResponse};
use crate::controller::respuesta::mongo::read::RespuestaRevisionMongo;
use crate::controller::revision::dto::RevisionListItemDTO;
use actix_web::{HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_revision::RespuestaRevision;
use serde_json::json;

pub struct ListarRevisionesController;

impl ListarRevisionesController {
    pub async fn list(pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /revisiones");

        let revision = RespuestaRevision::new(Box::new(RespuestaRevisionMongo::new(pool)));

        match revision.ejecutar(()).await {
            Ok(r) => {
                info!("GET /revisiones - {} resultados", r.len());

                let items: Vec<RevisionListItemDTO> = r
                    .into_iter()
                    .map(|rev| {
                        let mut links = Links::new();
                        links.insert(
                            "self".into(),
                            Link::get(format!("/revisiones/{}", rev.revision_id)),
                        );
                        links.insert(
                            "revisar".into(),
                            Link::post(format!("/revisiones/{}", rev.revision_id)),
                        );
                        links.insert(
                            "respuesta".into(),
                            Link::get(format!("/respuestas/{}", rev.revision_id)),
                        );
                        links.insert(
                            "postulante".into(),
                            Link::get(format!("/postulantes?id={}", rev.postulante_id)),
                        );

                        RevisionListItemDTO {
                            respuesta_id: rev.revision_id,
                            nombre_evaluacion: rev.nombre_evaluacion,
                            descripcion_evaluacion: rev.descripcion_evaluacion,
                            estado_revision: rev.estado_revision,
                            postulante_id: rev.postulante_id,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = Links::new();
                collection_links.insert("self".into(), Link::get("/revisiones"));

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!("GET /revisiones - error: {:?}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener revisiones"}))
            }
        }
    }
}
