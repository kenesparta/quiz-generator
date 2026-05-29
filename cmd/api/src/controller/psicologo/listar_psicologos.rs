use crate::controller::hateoas::{Link, Links, ListResponse};
use crate::controller::psicologo::mongo::read::PsicologoReadMongo;
use actix_web::{HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::psicologo::use_case::listar_psicologos::{InputData, ListarPsicologos};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct PsicologoListItemDTO {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub especialidad: String,
    pub colegiatura: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

pub struct ListarPsicologosController;

impl ListarPsicologosController {
    pub async fn list(pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /psicologos");

        let listar = ListarPsicologos::new(Box::new(PsicologoReadMongo::new(pool)));

        match listar.ejecutar(InputData).await {
            Ok(psicologos) => {
                info!("GET /psicologos - {} resultados", psicologos.len());

                let items: Vec<PsicologoListItemDTO> = psicologos
                    .into_iter()
                    .map(|p| {
                        let mut links = Links::new();
                        links.insert("self".into(), Link::get(format!("/psicologos/{}", p.id)));
                        PsicologoListItemDTO {
                            id: p.id,
                            nombre: p.nombre,
                            primer_apellido: p.primer_apellido,
                            segundo_apellido: p.segundo_apellido,
                            documento: p.documento,
                            especialidad: p.especialidad,
                            colegiatura: p.colegiatura,
                            links,
                        }
                    })
                    .collect();

                let mut collection_links = Links::new();
                collection_links.insert("self".into(), Link::get("/psicologos"));

                HttpResponse::Ok().json(ListResponse {
                    links: collection_links,
                    items,
                })
            }
            Err(e) => {
                error!("GET /psicologos - error: {}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al obtener psicologos"}))
            }
        }
    }
}
