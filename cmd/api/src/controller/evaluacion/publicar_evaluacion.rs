use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::publicar_evaluacion::{InputData, PublicarEvaluacion};

pub struct PublicarEvaluacionController;

impl PublicarEvaluacionController {
    pub async fn publicar(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let evaluacion_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("PATCH /evaluacion/publicar - id no proporcionado");
                return HttpResponse::BadRequest()
                    .json("no se esta enviando el id de la evaluacion");
            }
        };

        info!("PATCH /evaluacion/{}/publicar", evaluacion_id);

        let input = InputData {
            evaluacion_id: evaluacion_id.clone(),
        };

        let publicar_evaluacion = PublicarEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match publicar_evaluacion.ejecutar(input).await {
            Ok(_) => {
                info!(
                    "PATCH /evaluacion/{}/publicar - publicada exitosamente",
                    evaluacion_id
                );
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                error!(
                    "PATCH /evaluacion/{}/publicar - error: {:?}",
                    evaluacion_id, e
                );
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
