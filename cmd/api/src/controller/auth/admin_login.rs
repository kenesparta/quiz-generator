use crate::configuration::JwtSettings;
use crate::controller::auth::crypto::CifradoPorDefecto;
use crate::controller::auth::dto::{EmailLoginRequestDTO, LoginResponseDTO};
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::mongo::admin_read::AdminLoginMongo;
use crate::controller::auth::redis::admin_write::AdminLoginRedis;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};
use quizz_auth::admin::use_case::login::{InputData, LoginAdminPorEmail};
use quizz_common::use_case::CasoDeUso;

pub struct AdminLoginController;

impl AdminLoginController {
    pub async fn login(
        _req: HttpRequest,
        body: web::Json<EmailLoginRequestDTO>,
        pool: web::Data<mongodb::Client>,
        redis_client: web::Data<redis::Client>,
        jwt_settings: web::Data<JwtSettings>,
    ) -> HttpResponse {
        let dto = body.into_inner();
        info!("POST /login/admin - email={}", dto.email);

        let redis_impl = match AdminLoginRedis::new(redis_client) {
            Ok(r) => r,
            Err(e) => {
                error!("POST /login/admin - error al conectar con redis: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let use_case = LoginAdminPorEmail::new(
            Box::new(CifradoPorDefecto),
            Box::new(AdminLoginMongo::new(pool)),
            Box::new(redis_impl),
            Box::new(JWTProvider::new(
                jwt_settings.secret.clone(),
                jwt_settings.expiration_seconds,
            )),
        );

        match use_case
            .ejecutar(InputData {
                email: dto.email,
                password: dto.password,
            })
            .await
        {
            Ok(jwt_data) => {
                info!("POST /login/admin - login exitoso");
                let response_dto = LoginResponseDTO {
                    token: jwt_data.jwt_value,
                    expires_in: jwt_data.expiration,
                };
                HttpResponse::Ok().json(response_dto)
            }
            Err(e) => {
                error!("POST /login/admin - error en login: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
