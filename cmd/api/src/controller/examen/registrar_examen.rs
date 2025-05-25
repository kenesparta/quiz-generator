use crate::controller::examen::dto::RegistrarExamenDTO;
use actix_web::{HttpRequest, HttpResponse, web};

pub struct ExamenControlller;

impl ExamenControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarExamenDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
