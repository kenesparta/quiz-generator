use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::publicar_evaluacion::{InputData, PublicarEvaluacion};

pub struct PublicarEvaluacionController;

impl PublicarEvaluacionController {
    pub async fn publicar(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let input = InputData {
            evaluacion_id: match req.match_info().get("id") {
                Some(id) => id.to_string(),
                None => {
                    return HttpResponse::BadRequest()
                        .json("no se esta enviando el id de la evaluacion");
                }
            },
        };

        let publicar_evaluacion = PublicarEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match publicar_evaluacion.ejecutar(input).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
