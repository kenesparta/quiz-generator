use crate::controller::examen::dto::RegistrarExamenDTO;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

pub struct ExamenControlller;

impl ExamenControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarExamenDTO>,
        pool: web::Data<PgPool>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
