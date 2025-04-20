use actix_web::{HttpRequest, HttpResponse, Responder, web};

async fn handler(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health-check").route(web::put().to(handler)));
}
