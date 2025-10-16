use crate::controller::auth::postulante_login::PostulanteLoginController;
use actix_web::web;

pub fn postulante_login(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").service(
        web::resource("/postulante").route(web::post().to(PostulanteLoginController::login)),
    ));
}
