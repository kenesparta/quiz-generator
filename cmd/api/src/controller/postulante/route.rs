use crate::controller::postulante::obtener_postulante::{
    PostulanteGetController, PostulanteListGetController,
};
use crate::controller::postulante::registrar_postulante::PostulantePutController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulante")
            .service(web::resource("/{id}").route(web::put().to(PostulantePutController::put)))
            .service(web::resource("/{id}").route(web::get().to(PostulanteGetController::get)))
            .service(web::resource("/").route(web::get().to(PostulanteListGetController::get))),
    );
}
