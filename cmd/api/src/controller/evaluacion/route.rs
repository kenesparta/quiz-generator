use crate::controller::evaluacion::publicar_evaluacion::PublicarEvaluacionController;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionControlller;
use actix_web::web;

pub fn evaluacion(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluacion").service(
            web::resource("/{id}")
                .route(web::post().to(EvaluacionControlller::create))
                .route(web::put().to(EvaluacionControlller::asociar_examen))
                .route(web::patch().to(PublicarEvaluacionController::publicar)),
        ),
    );
}
