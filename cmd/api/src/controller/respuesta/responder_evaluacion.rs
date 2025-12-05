use crate::controller::respuesta::dto::{ActualizarEstadoDeEvaluacionDTO, ResponderEvaluacionDTO};
use crate::controller::respuesta::mongo::write::{
    RespositorioFinalizarEvaluacionMongo, RespuestaEvaluacionMongo,
};
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::use_case::finalizar_evaluacion::{
    FinalizarEvaluacion, InputData as InputDataFinEval,
};
use quizz_core::respuesta::use_case::responder_evaluacion::{InputData, ResponderEvaluacion};

pub struct ResponderEvaluacionController;

impl ResponderEvaluacionController {
    pub async fn response(
        req: HttpRequest,
        body: web::Json<ResponderEvaluacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("se debe enviar el ID del postulante");
            }
        };

        let dto = body.into_inner();
        let input = InputData {
            id,
            postulante_id: dto.postulante_id,
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

    // Actualizar estado de la evaluacion, en este caso solo se puede cambiar a finalizada
    pub async fn finalizar(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("se debe enviar el ID del postulante");
            }
        };

        let input = InputDataFinEval { id };
        let respuesta_fin_evaluacion =
            FinalizarEvaluacion::new(Box::new(RespositorioFinalizarEvaluacionMongo::new(pool)));
        match respuesta_fin_evaluacion.ejecutar(input).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("{:?}", e);
                HttpResponse::InternalServerError().json("Hubo un error")
            }
        }
    }
}
