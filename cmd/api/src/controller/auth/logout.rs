use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::middleware::extraer_token;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_auth::universal::use_case::logout::{InputData, Logout};
use quizz_common::use_case::CasoDeUso;
use tracing::{error, info, warn};

pub struct LogoutController;

impl LogoutController {
    pub async fn logout(
        req: HttpRequest,
        sesiones: web::Data<dyn Sesiones>,
        jwt: web::Data<JWTProvider>,
    ) -> HttpResponse {
        let token = match extraer_token(req.headers()) {
            Some(t) => t,
            None => {
                warn!("POST /logout - token no encontrado");
                return HttpResponse::Unauthorized().json("Token no encontrado");
            }
        };

        let claims = match jwt.verificar_token(token) {
            Ok(c) => c,
            Err(_) => {
                // Token expirado o invalido: el cliente igual debe limpiar su sesion.
                info!("POST /logout - token invalido o expirado, respondiendo OK");
                return HttpResponse::NoContent().finish();
            }
        };

        let use_case = Logout::new(sesiones.into_inner());

        match use_case
            .ejecutar(InputData {
                sujeto_id: claims.sub.clone(),
                sesion_id: claims.jti,
            })
            .await
        {
            Ok(_) => {
                info!("POST /logout - sesion cerrada, sub={}", claims.sub);
                HttpResponse::NoContent().finish()
            }
            Err(e) => {
                error!("POST /logout - error al cerrar sesion: {:?}", e);
                HttpResponse::InternalServerError().json("Error al cerrar sesion")
            }
        }
    }
}
