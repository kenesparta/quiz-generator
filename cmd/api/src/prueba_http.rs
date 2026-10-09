//! Soporte para tests HTTP en proceso: la aplicación real (rutas, autenticación, sesiones y
//! RBAC) sin base de datos. Una petición que pasa la autorización llega al handler y este
//! falla al no encontrar la base de datos, así que un 401/403/404/405 siempre viene de antes.
use crate::configuration::JwtSettings;
use crate::controller::auth::casbin_enforcer::crear_enforcer;
use crate::controller::auth::jwt::{Claims, JWTProvider};
use crate::startup::configurar_rutas;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::http::{Method, StatusCode};
use actix_web::{App, test, web};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::Sesiones;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) const SECRETO: &str = "secreto-de-prueba-con-al-menos-32-bytes";
pub(crate) const ID: &str = "0b9f1a52-3d4e-4f60-8a7b-9c0d1e2f3a4b";
pub(crate) const SESION: &str = "sesion-abierta";

/// Sesiones en memoria: `ID` tiene abierta la sesión `SESION`.
pub(crate) struct SesionesEnMemoria(Mutex<HashMap<String, String>>);

impl SesionesEnMemoria {
    pub(crate) fn con_sesion_abierta() -> Arc<dyn Sesiones> {
        let abiertas = HashMap::from([(ID.to_string(), SESION.to_string())]);
        Arc::new(Self(Mutex::new(abiertas)))
    }
}

#[async_trait::async_trait]
impl Sesiones for SesionesEnMemoria {
    async fn abrir(&self, sub: &str, jti: &str, _: u64) -> Result<(), LoginUniversalError> {
        self.0.lock().unwrap().insert(sub.into(), jti.into());
        Ok(())
    }

    async fn es_vigente(&self, sub: &str, jti: &str) -> Result<bool, LoginUniversalError> {
        Ok(self.0.lock().unwrap().get(sub).map(String::as_str) == Some(jti))
    }

    async fn cerrar(&self, sub: &str, _: &str) -> Result<(), LoginUniversalError> {
        self.0.lock().unwrap().remove(sub);
        Ok(())
    }

    async fn revocar(&self, sub: &str) -> Result<(), LoginUniversalError> {
        self.0.lock().unwrap().remove(sub);
        Ok(())
    }
}

/// Un almacén de sesiones caído.
pub(crate) struct SesionesCaidas;

#[async_trait::async_trait]
impl Sesiones for SesionesCaidas {
    async fn abrir(&self, _: &str, _: &str, _: u64) -> Result<(), LoginUniversalError> {
        Err(LoginUniversalError::ErrorGenericoCache)
    }

    async fn es_vigente(&self, _: &str, _: &str) -> Result<bool, LoginUniversalError> {
        Err(LoginUniversalError::ErrorGenericoCache)
    }

    async fn cerrar(&self, _: &str, _: &str) -> Result<(), LoginUniversalError> {
        Err(LoginUniversalError::ErrorGenericoCache)
    }

    async fn revocar(&self, _: &str) -> Result<(), LoginUniversalError> {
        Err(LoginUniversalError::ErrorGenericoCache)
    }
}

pub(crate) fn token_de_sesion(
    rol: &str,
    secreto: &str,
    algoritmo: Algorithm,
    vence_en: i64,
    jti: &str,
) -> String {
    let ahora = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: ID.to_string(),
        exp: ahora + vence_en,
        iat: ahora,
        jti: jti.to_string(),
        rol: Some(rol.to_string()),
    };
    encode(
        &Header::new(algoritmo),
        &claims,
        &EncodingKey::from_secret(secreto.as_bytes()),
    )
    .unwrap()
}

pub(crate) fn token_con(rol: &str, secreto: &str, algoritmo: Algorithm, vence_en: i64) -> String {
    token_de_sesion(rol, secreto, algoritmo, vence_en, SESION)
}

pub(crate) fn token(rol: Rol) -> String {
    token_con(&rol.to_string(), SECRETO, Algorithm::HS256, 3600)
}

pub(crate) async fn app() -> impl Service<
    actix_http::Request,
    Response = ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
> {
    app_con(SesionesEnMemoria::con_sesion_abierta()).await
}

pub(crate) async fn app_con(
    sesiones: Arc<dyn Sesiones>,
) -> impl Service<
    actix_http::Request,
    Response = ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
> {
    let enforcer = crear_enforcer().await.unwrap();
    let jwt = JWTProvider::new(&JwtSettings {
        secret: SECRETO.to_string(),
        expiration_seconds: 3600,
    });
    test::init_service(
        App::new()
            .app_data(web::Data::new(enforcer))
            .app_data(web::Data::new(jwt))
            .app_data(web::Data::from(sesiones))
            .configure(configurar_rutas),
    )
    .await
}

pub(crate) async fn estado(
    app: &impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
    >,
    metodo: Method,
    uri: &str,
    token: Option<&str>,
) -> StatusCode {
    let mut req = test::TestRequest::default().method(metodo).uri(uri);
    if let Some(token) = token {
        req = req.insert_header(("Authorization", format!("Bearer {token}")));
    }
    test::call_service(app, req.to_request()).await.status()
}
