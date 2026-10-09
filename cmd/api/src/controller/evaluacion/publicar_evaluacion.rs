use crate::controller::error::ApiError;
use crate::mongo::evaluacion::write::EvaluacionMongo;
use actix_web::{HttpResponse, web};
use quizz_core::evaluacion::use_case::publicar_evaluacion::{InputData, PublicarEvaluacion};

pub struct PublicarEvaluacionController;

impl PublicarEvaluacionController {
    pub async fn publicar(
        id: web::Path<String>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        PublicarEvaluacion::new(EvaluacionMongo::new(db))
            .ejecutar(InputData {
                evaluacion_id: id.into_inner(),
            })
            .await?;
        Ok(HttpResponse::Ok().finish())
    }
}
