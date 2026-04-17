use crate::controller::postulante::buscar_postulante::PostulanteObtenerPorDocumentoController;
use crate::controller::postulante::registrar_postulante::PostulanteController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulantes")
            .service(
                web::resource("")
                    .route(web::get().to(PostulanteObtenerPorDocumentoController::get))
                    .route(web::put().to(PostulanteController::update_by_documento)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::post().to(PostulanteController::create))
                    .route(web::delete().to(PostulanteController::remove)),
            ),
    );
}
