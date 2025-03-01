use crate::controller::applicant::applicant_put_controller::ApplicantPutController;
use actix_web::web;

pub fn applicant(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/applicants/{id}").route(web::put().to(ApplicantPutController::update)),
    );
}
