use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links, ListResponse, enlaces};
use crate::controller::respuesta::mongo::read::RespuestaRevisionMongo;
use crate::controller::revision::dto::RevisionListItemDTO;
use actix_web::{HttpResponse, web};
use quizz_core::respuesta::use_case::respuesta_revision::RespuestaRevision;

pub struct ListarRevisionesController;

impl ListarRevisionesController {
    pub async fn list(db: web::Data<mongodb::Database>) -> Result<HttpResponse, ApiError> {
        let r = RespuestaRevision::new(RespuestaRevisionMongo::new(db))
            .ejecutar()
            .await?;

        let items: Vec<RevisionListItemDTO> = r
            .into_iter()
            .map(|rev| {
                let links = enlaces::revision(&rev.revision_id, &rev.postulante_id);

                RevisionListItemDTO {
                    respuesta_id: rev.revision_id,
                    nombre_evaluacion: rev.nombre_evaluacion,
                    descripcion_evaluacion: rev.descripcion_evaluacion,
                    estado_revision: rev.estado_revision,
                    postulante_id: rev.postulante_id,
                    fecha_tiempo_fin: rev.fecha_tiempo_fin,
                    links,
                }
            })
            .collect();

        let mut collection_links = Links::new();
        collection_links.insert("self".into(), Link::get("/revisiones"));

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
