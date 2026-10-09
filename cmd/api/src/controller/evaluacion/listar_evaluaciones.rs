use crate::controller::error::ApiError;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::hateoas::{Link, Links, ListResponse, enlaces};
use actix_web::{HttpResponse, web};
use quizz_core::evaluacion::domain::value_object::evaluacion_estado::EvaluacionEstado;
use quizz_core::evaluacion::use_case::listar_evaluaciones::ListarEvaluaciones;
use serde::Serialize;
use std::str::FromStr;

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
    pub async fn list(db: web::Data<mongodb::Database>) -> Result<HttpResponse, ApiError> {
        let evaluaciones = ListarEvaluaciones::new(EvaluacionMongo::new(db))
            .ejecutar()
            .await?;

        let items: Vec<EvaluacionListItemDTO> = evaluaciones
            .into_iter()
            .map(|e| {
                let publicada = EvaluacionEstado::from_str(&e.estado)
                    .is_ok_and(|estado| estado == EvaluacionEstado::Publicado);
                let links = enlaces::evaluacion(&e.id, publicada);
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

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
