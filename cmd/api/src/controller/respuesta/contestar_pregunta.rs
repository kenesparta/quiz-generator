use crate::controller::auth::jwt::Claims;
use crate::controller::respuesta::dto::ContestacionDTO;
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::responder_evaluacion::{InputData, ResponderEvaluacion};
use serde_json::json;

pub struct ContestarPreguntaController;

impl ContestarPreguntaController {
    pub async fn contestar(
        req: HttpRequest,
        body: web::Json<ContestacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la respuesta"}));
            }
        };

        let examen_id = match req.match_info().get("examen_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID del examen"}));
            }
        };

        let pregunta_id = match req.match_info().get("pregunta_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la pregunta"}));
            }
        };

        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(c) => c,
            None => {
                return HttpResponse::Unauthorized().json(json!({"error": "Token no encontrado"}));
            }
        };

        let postulante_id = claims.sub.clone();

        info!(
            "POST /respuestas/{}/examenes/{}/preguntas/{}/contestaciones",
            respuesta_id, examen_id, pregunta_id
        );

        let dto = body.into_inner();
        let input = InputData {
            id: respuesta_id.clone(),
            postulante_id,
            evaluacion_id: String::new(), // Not needed, the repo uses respuesta_id
            examen_id: examen_id.clone(),
            pregunta_id: pregunta_id.clone(),
            respuestas: dto.respuestas,
        };

        let respuesta_questionario =
            ResponderEvaluacion::new(Box::new(RespuestaEvaluacionMongo::new(pool)));

        match respuesta_questionario.ejecutar(input).await {
            Ok(()) => {
                info!(
                    "POST /respuestas/{}/examenes/{}/preguntas/{}/contestaciones - guardada",
                    respuesta_id, examen_id, pregunta_id
                );
                HttpResponse::Ok().json(json!({
                    "mensaje": "Respuesta guardada correctamente",
                    "_links": {
                        "respuesta": {
                            "href": format!("/respuestas/{}", respuesta_id),
                            "method": "GET"
                        },
                        "finalizar": {
                            "href": format!("/respuestas/{}/estado", respuesta_id),
                            "method": "PATCH"
                        }
                    }
                }))
            }
            Err(err) => {
                error!(
                    "POST /respuestas/{}/examenes/{}/preguntas/{}/contestaciones - error: {:?}",
                    respuesta_id, examen_id, pregunta_id, err
                );
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al guardar la respuesta"}))
            }
        }
    }
}
