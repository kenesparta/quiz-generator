use crate::controller::examen::registrar_examen::ExamenControlller;
use crate::controller::pregunta::agregar_pregunta::AgregarPreguntaController;
use actix_web::web;

pub fn examen(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/examen")
            .service(web::resource("/{id}").route(web::post().to(ExamenControlller::create)))
            .service(web::scope("/{id}/pregunta").service(
                web::resource("").route(web::post().to(AgregarPreguntaController::create)),
            )),
    );
}
