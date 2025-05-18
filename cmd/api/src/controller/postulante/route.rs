use crate::controller::postulante::buscar_postulante::{
    PostulanteListController, PostulanteObtenerPorDocumentoController,
};
use crate::controller::postulante::registrar_postulante::PostulantePutController;
use actix_web::{HttpRequest, guard, web};

pub fn postulante(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postulantes")
            .service(web::resource("/{id}").route(web::put().to(PostulantePutController::put)))
            .service(
                web::resource("/search")
                    .route(web::get().to(PostulanteObtenerPorDocumentoController::get)),
            )
            .service(web::resource("").route(web::get().to(PostulanteListController::get))),
    );
}
