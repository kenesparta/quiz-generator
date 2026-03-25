use crate::controller::respuesta::dto::ResponderEvaluacionDTO;
use crate::controller::respuesta::mongo::write::{
    RespositorioFinalizarEvaluacionMongo, RespuestaEvaluacionMongo,
};
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
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
                warn!("PATCH /respuesta - id no proporcionado");
                return HttpResponse::BadRequest().json("se debe enviar el ID del postulante");
            }
        };

        info!("PATCH /respuesta/{}", id);

        let dto = body.into_inner();
        let input = InputData {
            id: id.clone(),
            postulante_id: dto.postulante_id,
            evaluacion_id: dto.evaluacion_id,
            examen_id: dto.examen_id,
            pregunta_id: dto.pregunta_id,
            respuestas: dto.respuestas,
        };

        let respuesta_questionario =
            ResponderEvaluacion::new(Box::new(RespuestaEvaluacionMongo::new(pool)));

        match respuesta_questionario.ejecutar(input).await {
            Ok(()) => {
                info!("PATCH /respuesta/{} - respuesta guardada", id);
                HttpResponse::Ok().finish()
            }
            Err(err) => {
                error!("PATCH /respuesta/{} - error: {:?}", id, err);
                HttpResponse::InternalServerError().json("Hubo un error")
            }
        }
    }

    pub async fn finalizar(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("PATCH /respuesta/finalizar - id no proporcionado");
                return HttpResponse::BadRequest().json("se debe enviar el ID del postulante");
            }
        };

        info!("PATCH /respuesta/{}/finalizar", id);

        let input = InputDataFinEval { id: id.clone() };
        let respuesta_fin_evaluacion =
            FinalizarEvaluacion::new(Box::new(RespositorioFinalizarEvaluacionMongo::new(pool)));
        match respuesta_fin_evaluacion.ejecutar(input).await {
            Ok(_) => {
                info!("PATCH /respuesta/{}/finalizar - finalizado", id);
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                error!("PATCH /respuesta/{}/finalizar - error: {:?}", id, e);
                HttpResponse::InternalServerError().json("Hubo un error")
            }
        }
    }
}
