use actix_web::{web, HttpRequest, HttpResponse, Responder};

async fn handler(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health-check").route(web::put().to(handler)));
}
