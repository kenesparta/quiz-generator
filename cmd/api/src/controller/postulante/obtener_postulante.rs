use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

pub struct PostulanteGetController;
impl PostulanteGetController {
    pub async fn get(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}

pub struct PostulanteListGetController;
impl PostulanteListGetController {
    pub async fn get(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
