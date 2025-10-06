use crate::controller::respuesta::asignar_evaluacion_postulante::AsignarEvaluacionPostulanteController;
use crate::controller::respuesta::responder_questionario::ResponderQuestionarioController;
use crate::controller::respuesta::respuesta_por_postulante::RespuestaPorPostulanteController;
use actix_web::web;

pub fn respuesta(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/respuesta")
            .service(
                web::resource("")
                    .route(web::post().to(AsignarEvaluacionPostulanteController::create)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(RespuestaPorPostulanteController::read))
                    .route(web::post().to(ResponderQuestionarioController::read)),
            ),
    );
}
