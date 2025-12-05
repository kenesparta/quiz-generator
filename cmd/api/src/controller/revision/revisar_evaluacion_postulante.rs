use crate::controller::revision::dto::RevisarEvaluacionPostulanteReviewDTO;
use crate::controller::revision::mongo::write::RevisionEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::realizar_revision::{
    InputData, InputDataExamen, RealizarRevision,
};

pub struct RevisarEvaluacionPostulanteController {}

impl RevisarEvaluacionPostulanteController {
    pub async fn review(
        _req: HttpRequest,
        body: web::Json<RevisarEvaluacionPostulanteReviewDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let body = body.into_inner();
        let input = InputData {
            respuesta_id: body.respuesta_id,
            evaluacion_id: body.evaluacion_id,
            examenes: body
                .examenes
                .into_iter()
                .map(|ex| InputDataExamen {
                    examen_id: ex.examen_id,
                    observacion: ex.observacion,
                })
                .collect(),
        };
        let revisar = RealizarRevision::new(Box::new(RevisionEvaluacionMongo::new(pool)));
        match revisar.ejecutar(input).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}
