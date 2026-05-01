use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::hateoas::{Link, Links, ListResponse};
use actix_web::{HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::examen::use_case::listar_examenes::{InputData, ListarExamenes};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ExamenListItemDTO {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub estado: String,
    pub cantidad_preguntas: usize,
    #[serde(rename = "_links")]
    pub links: Links,
}

pub struct ListarExamenesController;

impl ListarExamenesController {
    pub async fn list(pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /examenes");

        let listar = ListarExamenes::new(Box::new(ExamenMongo::new(pool)));

        match listar.ejecutar(InputData).await {
            Ok(examenes) => {
                info!("GET /examenes - {} resultados", examenes.len());

                let items: Vec<ExamenListItemDTO> = examenes
                    .into_iter()
                    .map(|e| {
                        let mut links = Links::new();
                        links.insert("self".into(), Link::get(format!("/examenes/{}", e.id)));
                        links.insert("update".into(), Link::put(format!("/examenes/{}", e.id)));
                        ExamenListItemDTO {
                            id: e.id,
                            titulo: e.titulo,
                            descripcion: e.descripcion,
                            instrucciones: e.instrucciones,
                            estado: e.estado,
                            cantidad_preguntas: e.cantidad_preguntas,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = Links::new();
                collection_links.insert("self".into(), Link::get("/examenes"));

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!("GET /examenes - error: {}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener examenes"}))
            }
        }
    }
}
