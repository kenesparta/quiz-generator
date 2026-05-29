use crate::controller::auth::logout::LogoutController;
use crate::controller::auth::universal_login::UniversalLoginController;
use actix_web::web;

pub fn login_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(UniversalLoginController::login)));
    cfg.service(web::resource("/logout").route(web::post().to(LogoutController::logout)));
}
