use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links, ListResponse};
use crate::controller::psicologo::mongo::write::PsicologoMongo;
use actix_web::{HttpResponse, web};
use quizz_core::psicologo::use_case::listar_psicologos::ListarPsicologos;
use serde::Serialize;

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
    pub async fn list(db: web::Data<mongodb::Database>) -> Result<HttpResponse, ApiError> {
        let psicologos = ListarPsicologos::new(PsicologoMongo::new(db))
            .ejecutar()
            .await?;

        let items: Vec<PsicologoListItemDTO> = psicologos
            .into_iter()
            .map(|p| {
                // No hay un endpoint para leer un psicólogo, así que no se enlaza ninguno.
                let links = Links::new();
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

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
