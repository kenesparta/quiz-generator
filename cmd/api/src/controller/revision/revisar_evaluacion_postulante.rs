use crate::controller::revision::dto::RevisarEvaluacionPostulanteReviewDTO;
use crate::controller::revision::mongo::write::RevisionEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};
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

        info!(
            "POST /revision - respuesta_id={}, evaluacion_id={}",
            body.respuesta_id, body.evaluacion_id
        );

        let input = InputData {
            respuesta_id: body.respuesta_id.clone(),
            evaluacion_id: body.evaluacion_id,
            resultado: body.resultado,
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
            Ok(_) => {
                info!(
                    "POST /revision - revision completada: {}",
                    body.respuesta_id
                );
                HttpResponse::Ok().finish()
            }
            Err(err) => {
                error!("POST /revision - error: {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
