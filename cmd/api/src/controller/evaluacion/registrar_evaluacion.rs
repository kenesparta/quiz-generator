use crate::controller::evaluacion::dto::RegistrarEvaluacionDTO;
use actix_web::{HttpRequest, HttpResponse, web};

pub struct EvaluacionControlller;

impl EvaluacionControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarEvaluacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }
}
