use crate::controller::evaluacion::dto::AgregarExamenesDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionControlller;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::agregar_examen::{AgregarExamenAEvaluacion, InputData};

impl EvaluacionControlller {
    pub async fn asociar_examen(
        req: HttpRequest,
        body: web::Json<AgregarExamenesDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let evaluacion_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("PUT /evaluacion - id no proporcionado");
                return HttpResponse::BadRequest()
                    .json("no se esta enviando el id de la evaluacion");
            }
        };

        info!("PUT /evaluacion/{}/examenes", evaluacion_id);

        let agregar_examenes = AgregarExamenAEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match agregar_examenes
            .ejecutar(InputData {
                evaluacion_id: evaluacion_id.clone(),
                examen_ids: body.into_inner().examenes,
            })
            .await
        {
            Ok(_) => {
                info!(
                    "PUT /evaluacion/{} - examenes asociados exitosamente",
                    evaluacion_id
                );
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                error!(
                    "PUT /evaluacion/{} - error al agregar examenes: {:?}",
                    evaluacion_id, e
                );
                HttpResponse::InternalServerError().json("error al agregar los examenes")
            }
        }
    }
}
