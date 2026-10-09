use crate::controller::admin::registrar_admin::AdminController;
use crate::controller::auth::middleware::Autorizacion;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

pub fn admin(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admins")
            .wrap(Autorizacion::para(Recurso::Admin))
            .service(web::resource("/{id}").route(web::post().to(AdminController::create))),
    );
}
