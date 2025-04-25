use crate::controller::postulante::registrar_postulante::ApplicantPutController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulante")
            .service(web::resource("/{id}").route(web::put().to(ApplicantPutController::update))),
    );
}
