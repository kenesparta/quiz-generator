use crate::configuration::JwtSettings;
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::redis::universal_borrar::LogoutUniversalRedis;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_auth::universal::use_case::logout::{InputData, Logout};
use quizz_common::use_case::CasoDeUso;

pub struct LogoutController;

impl LogoutController {
    pub async fn logout(
        req: HttpRequest,
        redis_client: web::Data<redis::Client>,
        jwt_settings: web::Data<JwtSettings>,
    ) -> HttpResponse {
        let token = match extraer_token(&req) {
            Some(t) => t,
            None => {
                warn!("POST /logout - token no encontrado");
                return HttpResponse::Unauthorized().json("Token no encontrado");
            }
        };

        let jwt_provider = JWTProvider::new(jwt_settings.secret.clone(), 0);
        let claims = match jwt_provider.verificar_token(&token) {
            Ok(c) => c,
            Err(_) => {
                // Token expirado o invalido: el cliente igual debe limpiar su sesion.
                info!("POST /logout - token invalido o expirado, respondiendo OK");
                return HttpResponse::NoContent().finish();
            }
        };

        let use_case = Logout::new(Box::new(LogoutUniversalRedis::new(redis_client)));

        match use_case
            .ejecutar(InputData {
                sujeto_id: claims.sub.clone(),
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

fn extraer_token(req: &HttpRequest) -> Option<String> {
    let auth_header = req.headers().get("Authorization")?.to_str().ok()?;

    auth_header
        .strip_prefix("Bearer ")
        .map(|token| token.to_string())
}
