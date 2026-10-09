use crate::controller::error::ApiError;
use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::hateoas::{Link, Links, ListResponse, enlaces};
use actix_web::{HttpResponse, web};
use quizz_core::examen::use_case::listar_examenes::ListarExamenes;
use serde::Serialize;

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
    pub async fn list(db: web::Data<mongodb::Database>) -> Result<HttpResponse, ApiError> {
        let examenes = ListarExamenes::new(ExamenMongo::new(db)).ejecutar().await?;

        let items: Vec<ExamenListItemDTO> = examenes
            .into_iter()
            .map(|e| {
                let links = enlaces::examen(&e.id);
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

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
