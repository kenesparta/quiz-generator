use crate::controller::respuesta::mongo::read::RespuestaPorPostulanteMongo;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_postulante::{InputData, RespuestaPorPostulante};
use serde_json::json;

pub struct RespuestaPorPostulanteController;

impl RespuestaPorPostulanteController {
    pub async fn read(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let postulante_id = match req.match_info().get("postulante_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("Se debe enviar el ID del postulante");
            }
        };

        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("Se debe enviar el ID de la respuesta");
            }
        };

        let resp_post =
            RespuestaPorPostulante::new(Box::new(RespuestaPorPostulanteMongo::new(pool)));
        let input = InputData { postulante_id, respuesta_id };
        match resp_post.ejecutar(input).await {
            Ok(r) => {
                let respuesta_dto: RespuestaDTO = r.into();
                HttpResponse::Ok().json(respuesta_dto)
            }
            Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
        }
    }
}
