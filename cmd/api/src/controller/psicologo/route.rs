use crate::controller::psicologo::registrar_psicologo::PsicologoController;
use actix_web::web;

pub fn psicologo(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/psicologo")
            .service(web::resource("/{id}").route(web::post().to(PsicologoController::create))),
    );
}
