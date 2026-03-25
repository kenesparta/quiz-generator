use crate::configuration::JwtSettings;
use crate::controller::auth::crypto::CifradoPorDefecto;
use crate::controller::auth::dto::{PostulanteLoginRequestDTO, PostulanteLoginResponseDTO};
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::mongo::read::PostulanteLoginMongo;
use crate::controller::auth::redis::write::PostulanteLoginRedis;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};
use quizz_auth::postulante::use_case::login::{InputData, LoginPostulantePorDocumento};
use quizz_common::use_case::CasoDeUso;

pub struct PostulanteLoginController;

impl PostulanteLoginController {
    pub async fn login(
        _req: HttpRequest,
        body: web::Json<PostulanteLoginRequestDTO>,
        pool: web::Data<mongodb::Client>,
        redis_client: web::Data<redis::Client>,
        jwt_settings: web::Data<JwtSettings>,
    ) -> HttpResponse {
        let dto = body.into_inner();
        info!("POST /login/postulante - documento={}", dto.user_name);

        let redis_impl = match PostulanteLoginRedis::new(redis_client) {
            Ok(r) => r,
            Err(e) => {
                error!(
                    "POST /login/postulante - error al conectar con redis: {:?}",
                    e
                );
                return HttpResponse::InternalServerError().finish();
            }
        };

        let use_case = LoginPostulantePorDocumento::new(
            Box::new(CifradoPorDefecto),
            Box::new(PostulanteLoginMongo::new(pool)),
            Box::new(redis_impl),
            Box::new(JWTProvider::new(
                jwt_settings.secret.clone(),
                jwt_settings.expiration_seconds,
            )),
        );

        match use_case
            .ejecutar(InputData {
                documento: dto.user_name,
                password: dto.password,
            })
            .await
        {
            Ok(jwt_data) => {
                info!("POST /login/postulante - login exitoso");
                let response_dto = PostulanteLoginResponseDTO {
                    token: jwt_data.jwt_value,
                    expires_in: jwt_data.expiration,
                };
                HttpResponse::Ok().json(response_dto)
            }
            Err(e) => {
                error!("POST /login/postulante - error en login: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
