use crate::controller::respuesta::asignar_evaluacion_postulante::AsignarEvaluacionPostulanteController;
use actix_web::web;

pub fn respuesta(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/respuesta").service(
        web::resource("").route(web::post().to(AsignarEvaluacionPostulanteController::create)),
    ));
}
