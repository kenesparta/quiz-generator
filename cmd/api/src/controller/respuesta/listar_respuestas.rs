use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links, ListResponse};
use crate::controller::respuesta::dto::{
    RespuestaListItemDTO, RespuestaQueryParams, build_respuesta_links,
};
use crate::controller::respuesta::mongo::read::ListaRespuestaPostulanteMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::lista_respuesta_postulante::{
    InputData, ListaRespuestaPostulante,
};

pub struct ListarRespuestasController;

impl ListarRespuestasController {
    /// `GET /respuestas`: las hojas sin finalizar de un postulante. El postulante ve siempre
    /// las suyas (se ignora la query); el personal debe indicar `postulante_id`.
    pub async fn list(
        req: HttpRequest,
        query: web::Query<RespuestaQueryParams>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let Some(rol) = claims.rol.as_deref().and_then(|r| r.parse::<Rol>().ok()) else {
            return Err(ApiError::prohibido("Rol no valido"));
        };

        let postulante_id = if rol == Rol::Postulante {
            claims.sub
        } else {
            query.into_inner().postulante_id.ok_or_else(|| {
                ApiError::solicitud_invalida("Se requiere el parametro postulante_id")
            })?
        };

        let respuestas =
            ListaRespuestaPostulante::new(Box::new(ListaRespuestaPostulanteMongo::new(db)))
                .ejecutar(InputData {
                    postulante_id: postulante_id.clone(),
                })
                .await?;

        let items: Vec<RespuestaListItemDTO> = respuestas
            .into_iter()
            .map(|r| RespuestaListItemDTO {
                links: build_respuesta_links(&r.respuesta_id, r.estado, rol),
                id: r.respuesta_id,
                nombre_evaluacion: r.nombre_evaluacion,
                descripcion_evaluacion: r.descripcion_evaluacion,
                estado: r.estado.to_string(),
            })
            .collect();

        let mut collection_links = Links::new();
        collection_links.insert(
            "self".into(),
            Link::get(format!("/respuestas?postulante_id={postulante_id}")),
        );

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
