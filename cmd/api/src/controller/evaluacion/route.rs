use crate::controller::evaluacion::publicar_evaluacion::PublicarEvaluacionController;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionControlller;
use crate::controller::respuesta::asignar_evaluacion_postulante::AsignarEvaluacionPostulanteController;
use actix_web::web;

pub fn evaluacion(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluaciones")
            .service(
                web::resource("/{id}")
                    .route(web::post().to(EvaluacionControlller::create))
                    .route(web::put().to(EvaluacionControlller::asociar_examen))
                    .route(web::patch().to(PublicarEvaluacionController::publicar)),
            )
            .service(
                web::resource("/{evaluacion_id}/respuestas")
                    .route(web::post().to(AsignarEvaluacionPostulanteController::create)),
            ),
    );
}
