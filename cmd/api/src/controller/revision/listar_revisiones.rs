use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links, ListResponse, enlaces};
use crate::controller::revision::dto::RevisionListItemDTO;
use crate::mongo::respuesta::RespuestaMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_core::respuesta::use_case::respuesta_revision::RespuestaRevision;

pub struct ListarRevisionesController;

impl ListarRevisionesController {
    pub async fn list(
        req: HttpRequest,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(rol) = req
            .extensions()
            .get::<Claims>()
            .and_then(|claims| claims.rol.as_deref()?.parse::<Rol>().ok())
        else {
            return Err(ApiError::prohibido("Rol no valido"));
        };
        let r = RespuestaRevision::new(RespuestaMongo::new(db))
            .ejecutar()
            .await?;

        let items: Vec<RevisionListItemDTO> = r
            .into_iter()
            .map(|rev| {
                let links = enlaces::revision(&rev.revision_id, &rev.postulante_id, rol);

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
