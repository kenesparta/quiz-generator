use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::respuesta::dto::ContestacionDTO;
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::responder_evaluacion::{InputData, ResponderEvaluacion};
use serde_json::json;

pub struct ContestarPreguntaController;

impl ContestarPreguntaController {
    /// Guarda la contestación del postulante dueño de la hoja (el dueño sale del token). Una hoja
    /// ajena responde 404 y una que no está en proceso, 409.
    pub async fn contestar(
        req: HttpRequest,
        ruta: web::Path<(String, String, String)>,
        body: web::Json<ContestacionDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let (respuesta_id, examen_id, pregunta_id) = ruta.into_inner();

        ResponderEvaluacion::new(Box::new(RespuestaEvaluacionMongo::new(db)))
            .ejecutar(InputData {
                id: respuesta_id.clone(),
                postulante_id: claims.sub,
                examen_id,
                pregunta_id,
                respuestas: body.into_inner().respuestas,
            })
            .await?;

        Ok(HttpResponse::Ok().json(json!({
            "mensaje": "Respuesta guardada correctamente",
            "_links": {
                "respuesta": {
                    "href": format!("/respuestas/{respuesta_id}"),
                    "method": "GET"
                },
                "finalizar": {
                    "href": format!("/respuestas/{respuesta_id}/estado"),
                    "method": "PATCH"
                }
            }
        })))
    }
}
