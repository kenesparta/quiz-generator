use crate::controller::auth::middleware::Autorizacion;
use crate::controller::evaluacion::listar_evaluaciones::ListarEvaluacionesController;
use crate::controller::evaluacion::publicar_evaluacion::PublicarEvaluacionController;
use crate::controller::evaluacion::registrar_evaluacion::EvaluacionController;
use crate::controller::respuesta::asignar_evaluacion_postulante::AsignarEvaluacionPostulanteController;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

pub fn evaluacion(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluaciones")
            .wrap(Autorizacion::para(Recurso::Evaluacion))
            .service(web::resource("").route(web::get().to(ListarEvaluacionesController::list)))
            .service(
                web::resource("/{id}")
                    .route(web::post().to(EvaluacionController::create))
                    .route(web::put().to(EvaluacionController::asociar_examen))
                    .route(web::patch().to(PublicarEvaluacionController::publicar)),
            )
            .service(
                web::resource("/{evaluacion_id}/respuestas")
                    .route(web::post().to(AsignarEvaluacionPostulanteController::create)),
            ),
    );
}
