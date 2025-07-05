use crate::controller::evaluacion::registrar_evaluacion::EvaluacionControlller;
use actix_web::web;

pub fn evaluacion(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluacion")
            .service(web::resource("/{id}").route(web::post().to(EvaluacionControlller::create)))
            .service(web::resource("/{id}/examen").route(web::post().to(EvaluacionControlller::create))),
    );
}
