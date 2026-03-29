use crate::controller::admin::registrar_admin::AdminController;
use actix_web::web;

pub fn admin(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admins")
            .service(web::resource("/{id}").route(web::post().to(AdminController::create))),
    );
}
