use crate::controller::respuesta::dto::ResponderEvaluacionDTO;
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::responder_evaluacion::{InputData, ResponderEvaluacion};

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
        let input = InputData {
            id: dto.id,
            postulante_id,
            evaluacion_id: dto.evaluacion_id,
            examen_id: dto.examen_id,
            pregunta_id: dto.pregunta_id,
            respuestas: dto.respuestas,
        };

        let respuesta_questionario =
            ResponderEvaluacion::new(Box::new(RespuestaEvaluacionMongo::new(pool)));

        match respuesta_questionario.ejecutar(input).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(err) => {
                println!("{:?}", err);
                HttpResponse::InternalServerError().json("Hubo un error")
            }
        }
    }
}
