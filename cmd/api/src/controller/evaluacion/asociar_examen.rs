use crate::controller::evaluacion::dto::AgregarExamenesDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionControlller;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::agregar_examen::{AgregarExamenAEvaluacion, InputData};
use std::sync::Arc;

impl EvaluacionControlller {
    pub async fn asociar_examen(
        req: HttpRequest,
        body: web::Json<AgregarExamenesDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let agregar_examenes = AgregarExamenAEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match agregar_examenes
            .ejecutar(InputData {
                evaluacion_id: match req.match_info().get("id") {
                    Some(id) => id.to_string(),
                    None => {
                        return HttpResponse::BadRequest()
                            .json("no se esta enviando el id de la evaluacion");
                    }
                },
                examen_ids: body.into_inner().examenes,
            })
            .await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().json("error al agregar los examenes"),
        }
    }
}
