use crate::controller::respuesta::dto::ResponderEvaluacionDTO;
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_evaluacion::RespuestaEvaluacion;

pub struct RespuestaEvaluacionController;

impl RespuestaEvaluacionController {
    pub async fn read(
        req: HttpRequest,
        body: web::Json<ResponderEvaluacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let postulante_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("se debe enviar el ID del postulante");
            }
        };
        let dto = body.into_inner();
        let respuesta_questionario =
            RespuestaEvaluacion::new(Box::new(RespuestaEvaluacionMongo::new(pool)));
        // respuesta_questionario.ejecutar(dto)
        HttpResponse::Ok().json("Respuesta enviada correctamente")
    }
}
