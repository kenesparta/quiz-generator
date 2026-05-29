use crate::controller::psicologo::listar_psicologos::ListarPsicologosController;
use crate::controller::psicologo::registrar_psicologo::PsicologoController;
use actix_web::web;

pub fn psicologo(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/psicologos")
            .service(web::resource("").route(web::get().to(ListarPsicologosController::list)))
            .service(web::resource("/{id}").route(web::post().to(PsicologoController::create))),
    );
}
