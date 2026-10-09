use crate::controller::error::ApiError;
use crate::controller::evaluacion::dto::RegistrarEvaluacionDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use actix_web::{HttpResponse, web};
use quizz_core::evaluacion::use_case::crear_evaluacion::{CrearEvaluacion, InputData};

pub struct EvaluacionController;

impl EvaluacionController {
    pub async fn create(
        id: web::Path<String>,
        body: web::Json<RegistrarEvaluacionDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        CrearEvaluacion::new(EvaluacionMongo::new(db))
            .ejecutar(InputData {
                id: id.into_inner(),
                titulo: dto.titulo,
                descripcion: dto.descripcion,
            })
            .await?;
        Ok(HttpResponse::Created().finish())
    }
}
