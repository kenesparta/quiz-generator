use crate::controller::respuesta::mongo::read::RespuestaPorPostulanteMongo;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_postulante::{InputData, RespuestaPorPostulante};
use serde_json::json;

pub struct RespuestaPorPostulanteController;

impl RespuestaPorPostulanteController {
    pub async fn read(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let postulante_id = match req.match_info().get("postulante_id") {
            Some(id) => id.to_string(),
            None => {
                warn!("GET /respuesta/postulante - postulante_id no proporcionado");
                return HttpResponse::BadRequest().json("Se debe enviar el ID del postulante");
            }
        };

        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("GET /respuesta - id no proporcionado");
                return HttpResponse::BadRequest().json("Se debe enviar el ID de la respuesta");
            }
        };

        info!(
            "GET /respuesta/{}/postulante/{}",
            respuesta_id, postulante_id
        );

        let resp_post =
            RespuestaPorPostulante::new(Box::new(RespuestaPorPostulanteMongo::new(pool)));
        let input = InputData {
            postulante_id: postulante_id.clone(),
            respuesta_id: respuesta_id.clone(),
        };
        match resp_post.ejecutar(input).await {
            Ok(r) => {
                info!(
                    "GET /respuesta/{}/postulante/{} - encontrado",
                    respuesta_id, postulante_id
                );
                let respuesta_dto: RespuestaDTO = r.into();
                HttpResponse::Ok().json(respuesta_dto)
            }
            Err(e) => {
                error!(
                    "GET /respuesta/{}/postulante/{} - error: {}",
                    respuesta_id, postulante_id, e
                );
                HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
            }
        }
    }
}
