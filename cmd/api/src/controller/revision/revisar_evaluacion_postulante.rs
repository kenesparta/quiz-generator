use crate::controller::hateoas::Link;
use crate::controller::revision::dto::{CrearRevisionDTO, RevisionCreatedDTO};
use crate::controller::revision::mongo::write::RevisionEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::realizar_revision::{
    InputData, InputDataExamen, RealizarRevision,
};
use serde_json::json;

pub struct RevisarEvaluacionPostulanteController;

impl RevisarEvaluacionPostulanteController {
    pub async fn review(
        req: HttpRequest,
        body: web::Json<CrearRevisionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let respuesta_id = match req.match_info().get("revision_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la respuesta"}));
            }
        };

        let body = body.into_inner();

        info!(
            "POST /revisiones/{} - evaluacion_id={}",
            respuesta_id, body.evaluacion_id
        );

        let input = InputData {
            respuesta_id: respuesta_id.clone(),
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
                info!("POST /revisiones/{} - revision completada", respuesta_id);

                let mut links = crate::controller::hateoas::Links::new();
                links.insert(
                    "self".into(),
                    Link::get(format!("/revisiones/{}", respuesta_id)),
                );
                links.insert(
                    "respuesta".into(),
                    Link::get(format!("/respuestas/{}", respuesta_id)),
                );

                HttpResponse::Created().json(RevisionCreatedDTO {
                    respuesta_id,
                    estado_revision: "Finalizada".to_string(),
                    links,
                })
            }
            Err(err) => {
                error!("POST /revisiones/{} - error: {}", respuesta_id, err);
                HttpResponse::InternalServerError()
                    .json(json!({"error": "Error al realizar la revision"}))
            }
        }
    }
}
