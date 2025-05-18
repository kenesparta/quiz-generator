use crate::controller::postulante::obtener_postulante::PostulanteObtenerPorDocumentoController;
use crate::controller::postulante::registrar_postulante::PostulantePutController;
use actix_web::web;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulantes")
            .service(web::resource("/{id}").route(web::put().to(PostulantePutController::put)))
            .service(
                web::resource("")
                    .route(web::get().to(PostulanteObtenerPorDocumentoController::get)),
            ),
    );
}
