use crate::controller::applicant::applicant_put::ApplicantPutController;
use actix_web::web;

pub fn applicant(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/applicant")
            .service(web::resource("/{id}").route(web::put().to(ApplicantPutController::update))),
    );
}
