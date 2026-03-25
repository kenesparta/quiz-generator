use crate::controller::auth::admin_login::AdminLoginController;
use crate::controller::auth::postulante_login::PostulanteLoginController;
use crate::controller::auth::psicologo_login::PsicologoLoginController;
use actix_web::web;

pub fn login_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .service(
                web::resource("/postulante")
                    .route(web::post().to(PostulanteLoginController::login)),
            )
            .service(
                web::resource("/psicologo").route(web::post().to(PsicologoLoginController::login)),
            )
            .service(web::resource("/admin").route(web::post().to(AdminLoginController::login))),
    );
}
