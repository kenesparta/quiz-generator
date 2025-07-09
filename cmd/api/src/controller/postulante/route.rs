use crate::controller::postulante::buscar_postulante::PostulanteObtenerPorDocumentoController;
use crate::controller::postulante::registrar_postulante::PostulanteController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulante")
            .service(
                web::resource("")
                    .route(web::get().to(PostulanteObtenerPorDocumentoController::get)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::post().to(PostulanteController::create))
                    .route(web::put().to(PostulanteController::update))
                    .route(web::delete().to(PostulanteController::remove)),
            ),
    );
}
