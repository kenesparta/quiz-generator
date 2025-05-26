use crate::controller::pregunta::agregar_pregunta::AgregarPreguntaController;
use actix_web::web;

pub fn pregunta(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/pregunta").service(
        web::resource("/examen/{id}").route(web::post().to(AgregarPreguntaController::create)),
    ));
}
