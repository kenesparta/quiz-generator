use crate::controller::usuario::handler::UsuarioController;
use actix_web::web;

pub fn usuario(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/usuario")
            .service(web::resource("").route(web::post().to(UsuarioController::create))),
    );
}
