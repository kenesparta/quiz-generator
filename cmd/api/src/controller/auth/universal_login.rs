use crate::controller::auth::crypto::CifradoPorDefecto;
use crate::controller::auth::dto::{DocumentoLoginRequestDTO, LoginResponseDTO};
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::mongo::universal_read::LoginUniversalMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_auth::universal::use_case::login::{InputData, LoginUniversal};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::value_object::documento::Documento;

pub struct UniversalLoginController;

impl UniversalLoginController {
    pub async fn login(
        _req: HttpRequest,
        body: web::Json<DocumentoLoginRequestDTO>,
        pool: web::Data<mongodb::Database>,
        sesiones: web::Data<dyn Sesiones>,
        jwt: web::Data<JWTProvider>,
    ) -> HttpResponse {
        let dto = body.into_inner();
        info!("POST /login");

        let documento = match Documento::new(&dto.documento) {
            Ok(d) => d,
            Err(e) => {
                warn!("POST /login - documento no válido: {:?}", e);
                return HttpResponse::BadRequest().json("Documento no válido");
            }
        };

        let use_case = LoginUniversal::new(
            Box::new(CifradoPorDefecto),
            Box::new(LoginUniversalMongo::new(pool)),
            sesiones.into_inner(),
            Box::new(jwt.get_ref().clone()),
        );

        match use_case
            .ejecutar(InputData {
                documento: documento.value().clone(),
                password: dto.password,
            })
            .await
        {
            Ok(jwt_data) => {
                info!("POST /login - login exitoso, rol={}", jwt_data.rol);
                let response_dto = LoginResponseDTO {
                    token: jwt_data.jwt_value,
                    expires_in: jwt_data.expiration,
                    rol: jwt_data.rol,
                };
                HttpResponse::Ok().json(response_dto)
            }
            Err(LoginUniversalError::UsuarioNoEncontrado) => {
                warn!("POST /login - usuario no encontrado");
                HttpResponse::Unauthorized().json("Documento o password incorrectos")
            }
            Err(LoginUniversalError::PasswordIncorrecto) => {
                warn!("POST /login - password incorrecto");
                HttpResponse::Unauthorized().json("Documento o password incorrectos")
            }
            Err(e) => {
                error!("POST /login - error en login: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
