use crate::controller::auth::middleware::Autorizacion;
use crate::controller::postulante::buscar_postulante::PostulanteObtenerPorDocumentoController;
use crate::controller::postulante::registrar_postulante::PostulanteController;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulantes")
            .wrap(Autorizacion::para(Recurso::Postulante))
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
