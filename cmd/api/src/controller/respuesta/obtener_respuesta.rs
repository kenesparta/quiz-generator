use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::respuesta::dto::RespuestaDetailDTO;
use crate::controller::respuesta::mongo::read::RespuestaLecturaMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::domain::value_objects::zona_horaria::ahora_lima;
use quizz_core::respuesta::use_case::respuesta_postulante::{InputData, ObtenerRespuesta};

pub struct ObtenerRespuestaController;

impl ObtenerRespuestaController {
    /// `GET /respuestas/{id}`: el postulante solo obtiene su propia hoja (otra responde 404) y
    /// sin puntos; el personal obtiene cualquiera, completa.
    pub async fn get(
        req: HttpRequest,
        respuesta_id: web::Path<String>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let Some(rol) = claims.rol.as_deref().and_then(|r| r.parse::<Rol>().ok()) else {
            return Err(ApiError::prohibido("Rol no valido"));
        };

        let respuesta = ObtenerRespuesta::new(RespuestaLecturaMongo::new(db))
            .ejecutar(InputData {
                respuesta_id: respuesta_id.into_inner(),
                postulante_id: (rol == Rol::Postulante).then_some(claims.sub),
                ahora: ahora_lima(),
            })
            .await?;

        Ok(HttpResponse::Ok().json(RespuestaDetailDTO::para(rol, respuesta)))
    }
}
