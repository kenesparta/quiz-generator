use crate::controller::error::ApiError;
use crate::controller::evaluacion::dto::AgregarExamenesDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionController;
use actix_web::{HttpResponse, web};
use quizz_core::evaluacion::use_case::agregar_examen::{AgregarExamenAEvaluacion, InputData};

impl EvaluacionController {
    pub async fn asociar_examen(
        id: web::Path<String>,
        body: web::Json<AgregarExamenesDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        AgregarExamenAEvaluacion::new(EvaluacionMongo::new(db))
            .ejecutar(InputData {
                evaluacion_id: id.into_inner(),
                examen_ids: body.into_inner().examenes,
            })
            .await?;
        Ok(HttpResponse::Ok().finish())
    }
}
