use crate::controller::postulante::buscar_postulante::{
    PostulanteListController, PostulanteObtenerPorDocumentoController,
};
use crate::controller::postulante::registrar_postulante::PostulanteController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulantes")
            .service(web::resource("/{id}").route(web::post().to(PostulanteController::create)))
            .service(web::resource("/{id}").route(web::put().to(PostulanteController::update)))
            .service(
                web::resource("/search")
                    .route(web::get().to(PostulanteObtenerPorDocumentoController::get)),
            )
            .service(web::resource("").route(web::get().to(PostulanteListController::get))),
    );
}
