use crate::controller::auth::dto::{DocumentoLoginRequestDTO, LoginResponseDTO};
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::limite_intentos::LimiteDeIntentos;
use crate::controller::auth::mongo::universal_read::LoginUniversalMongo;
use crate::controller::cifrado::Bcrypt;
use crate::controller::error::ApiError;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_auth::universal::use_case::login::{InputData, LoginUniversal};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::value_object::documento::Documento;
use tracing::{error, info, warn};

pub struct UniversalLoginController;

impl UniversalLoginController {
    /// `POST /login`. Limita los intentos por IP y los fallos por documento (429 con
    /// `Retry-After`); si Redis no responde, no deja intentar (503).
    pub async fn login(
        req: HttpRequest,
        body: web::Json<DocumentoLoginRequestDTO>,
        db: web::Data<mongodb::Database>,
        sesiones: web::Data<dyn Sesiones>,
        jwt: web::Data<JWTProvider>,
        limite: web::Data<LimiteDeIntentos>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        let documento = Documento::new(&dto.documento)
            .map_err(|_| ApiError::solicitud_invalida("Documento no válido"))?;
        let documento = documento.value().clone();

        let redis_caido = |e: redis::RedisError| {
            error!("POST /login - limite de intentos no disponible: {e}");
            ApiError::NoDisponible
        };
        let demasiados = ApiError::DemasiadosIntentos {
            reintentar_en: limite.ventana_segundos(),
        };
        if !limite.permitir_intento(&req).await.map_err(redis_caido)? {
            warn!("POST /login - demasiados intentos desde la misma IP");
            return Err(demasiados);
        }
        if limite
            .documento_bloqueado(&documento)
            .await
            .map_err(redis_caido)?
        {
            warn!("POST /login - demasiados fallos para un mismo documento");
            return Err(demasiados);
        }

        let resultado = LoginUniversal::new(
            Box::new(Bcrypt::default()),
            Box::new(LoginUniversalMongo::new(db)),
            sesiones.into_inner(),
            Box::new(jwt.get_ref().clone()),
        )
        .ejecutar(InputData {
            documento: documento.clone(),
            password: dto.password,
        })
        .await;

        let sesion = match resultado {
            Ok(sesion) => sesion,
            Err(
                e @ (LoginUniversalError::UsuarioNoEncontrado
                | LoginUniversalError::PasswordIncorrecto),
            ) => {
                limite
                    .registrar_fallo(&documento)
                    .await
                    .map_err(redis_caido)?;
                return Err(e.into());
            }
            Err(e) => return Err(e.into()),
        };

        info!("POST /login - login exitoso, rol={}", sesion.rol);
        Ok(HttpResponse::Ok().json(LoginResponseDTO {
            token: sesion.jwt_value,
            expires_in: sesion.expiration,
            rol: sesion.rol,
        }))
    }
}
