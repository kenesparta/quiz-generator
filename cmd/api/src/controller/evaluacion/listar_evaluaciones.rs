use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::hateoas::{Link, Links, ListResponse};
use actix_web::{HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::listar_evaluaciones::{InputData, ListarEvaluaciones};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct EvaluacionListItemDTO {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub estado: String,
    pub esta_activo: String,
    pub cantidad_examenes: usize,
    #[serde(rename = "_links")]
    pub links: Links,
}

pub struct ListarEvaluacionesController;

impl ListarEvaluacionesController {
    pub async fn list(pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /evaluaciones");

        let listar = ListarEvaluaciones::new(Box::new(EvaluacionMongo::new(pool)));

        match listar.ejecutar(InputData).await {
            Ok(evaluaciones) => {
                info!("GET /evaluaciones - {} resultados", evaluaciones.len());

                let items: Vec<EvaluacionListItemDTO> = evaluaciones
                    .into_iter()
                    .map(|e| {
                        let mut links = Links::new();
                        links.insert("self".into(), Link::get(format!("/evaluaciones/{}", e.id)));
                        links.insert(
                            "asociar_examenes".into(),
                            Link::put(format!("/evaluaciones/{}", e.id)),
                        );
                        links.insert(
                            "publicar".into(),
                            Link::patch(format!("/evaluaciones/{}", e.id)),
                        );
                        EvaluacionListItemDTO {
                            id: e.id,
                            nombre: e.nombre,
                            descripcion: e.descripcion,
                            estado: e.estado,
                            esta_activo: e.esta_activo,
                            cantidad_examenes: e.cantidad_examenes,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = Links::new();
                collection_links.insert("self".into(), Link::get("/evaluaciones"));

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!("GET /evaluaciones - error: {}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener evaluaciones"}))
            }
        }
    }
}
