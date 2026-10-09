use crate::controller::auth::crypto::CifradoPorDefecto;
use crate::controller::auth::dto::{DocumentoLoginRequestDTO, LoginResponseDTO};
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::mongo::universal_read::LoginUniversalMongo;
use crate::controller::error::ApiError;
use actix_web::{HttpResponse, web};
use log::info;
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_auth::universal::use_case::login::{InputData, LoginUniversal};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::value_object::documento::Documento;

pub struct UniversalLoginController;

impl UniversalLoginController {
    pub async fn login(
        body: web::Json<DocumentoLoginRequestDTO>,
        db: web::Data<mongodb::Database>,
        sesiones: web::Data<dyn Sesiones>,
        jwt: web::Data<JWTProvider>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        let documento = Documento::new(&dto.documento)
            .map_err(|_| ApiError::solicitud_invalida("Documento no válido"))?;

        let sesion = LoginUniversal::new(
            Box::new(CifradoPorDefecto),
            Box::new(LoginUniversalMongo::new(db)),
            sesiones.into_inner(),
            Box::new(jwt.get_ref().clone()),
        )
        .ejecutar(InputData {
            documento: documento.value().clone(),
            password: dto.password,
        })
        .await?;

        info!("POST /login - login exitoso, rol={}", sesion.rol);
        Ok(HttpResponse::Ok().json(LoginResponseDTO {
            token: sesion.jwt_value,
            expires_in: sesion.expiration,
            rol: sesion.rol,
        }))
    }
}
