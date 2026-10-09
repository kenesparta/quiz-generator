//! Autenticación y autorización de las rutas protegidas.
//!
//! - [`AuthMiddleware`] envuelve todo el scope protegido: exige un JWT válido con un rol
//!   conocido cuya sesión siga abierta, y deja los [`Claims`] en las extensiones de la petición.
//!   Sin eso responde 401/403 (503 si no se puede consultar la sesión). Necesita
//!   `web::Data<JWTProvider>` y `web::Data<dyn Sesiones>` registrados como `app_data`.
//! - [`Autorizacion`] envuelve cada scope de rutas con el [`Recurso`] que ese scope expone y
//!   consulta la política RBAC (rol × recurso × acción).
//!
//! El recurso se declara junto a la ruta y nunca se deduce de la URL: el router decodifica la
//! ruta (`/%61dmins` llega a `/admins`), así que cualquier análisis de texto distinto al suyo
//! permite saltarse la autorización. Todo lo que no se puede clasificar se deniega.
use crate::controller::auth::casbin_enforcer::CasbinAutorizacion;
use crate::controller::auth::jwt::{Claims, JWTProvider};
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpMessage, HttpResponse, web};
use casbin::Enforcer;
use futures::future::{LocalBoxFuture, Ready, ok};
use log::{debug, error, info, warn};
use quizz_auth::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use quizz_auth::autorizacion::domain::error::autorizacion::AutorizacionError;
use quizz_auth::autorizacion::domain::value_object::accion::Accion;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_auth::autorizacion::provider::autorizacion::AutorizacionVerificar;
use quizz_auth::universal::provider::repositorio::Sesiones;
use std::rc::Rc;

/// Autenticación: exige `Authorization: Bearer <jwt>` válido en todas las rutas que envuelve.
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            info!("{} {}", req.method(), req.path());

            match autenticar(&req).await {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(service.call(req).await?.map_into_left_body())
                }
                Err(rechazo) => Ok(req.into_response(rechazo.respuesta()).map_into_right_body()),
            }
        })
    }
}

/// Motivo por el que el middleware corta una petición. Se traduce a HTTP en un solo lugar.
enum Rechazo {
    NoAutenticado(&'static str),
    Denegado(&'static str),
    Interno,
    NoDisponible,
}

impl Rechazo {
    fn respuesta(self) -> HttpResponse {
        match self {
            Rechazo::NoAutenticado(motivo) => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": motivo }))
            }
            Rechazo::Denegado(motivo) => {
                HttpResponse::Forbidden().json(serde_json::json!({ "error": motivo }))
            }
            Rechazo::Interno => HttpResponse::InternalServerError().finish(),
            Rechazo::NoDisponible => HttpResponse::ServiceUnavailable().finish(),
        }
    }
}

async fn autenticar(req: &ServiceRequest) -> Result<Claims, Rechazo> {
    let Some(token) = extraer_token(req.headers()) else {
        warn!("{} {} - token no encontrado", req.method(), req.path());
        return Err(Rechazo::NoAutenticado("Token no encontrado"));
    };
    let Some(jwt) = req.app_data::<web::Data<JWTProvider>>() else {
        error!("el proveedor JWT no esta registrado como app_data");
        return Err(Rechazo::Interno);
    };

    let claims = jwt.verificar_token(token).map_err(|e| {
        warn!("{} {} - {}", req.method(), req.path(), e);
        Rechazo::NoAutenticado("Token no valido o expirado")
    })?;

    if rol_de(&claims).is_none() {
        warn!(
            "{} {} - rol ausente o no valido, sub={}",
            req.method(),
            req.path(),
            claims.sub
        );
        return Err(Rechazo::Denegado("Rol no valido"));
    }

    // Un token bien firmado solo autentica mientras su sesión siga abierta: así /logout
    // revoca el token antes de su `exp`. Si no se puede consultar, se rechaza (falla cerrado).
    let Some(sesiones) = req.app_data::<web::Data<dyn Sesiones>>() else {
        error!("las sesiones no estan registradas como app_data");
        return Err(Rechazo::Interno);
    };
    match sesiones.es_vigente(&claims.sub, &claims.jti).await {
        Ok(true) => Ok(claims),
        Ok(false) => {
            warn!(
                "{} {} - sesion cerrada o reemplazada, sub={}",
                req.method(),
                req.path(),
                claims.sub
            );
            Err(Rechazo::NoAutenticado("Sesion cerrada o expirada"))
        }
        Err(e) => {
            error!("no se pudo consultar la sesion: {e}");
            Err(Rechazo::NoDisponible)
        }
    }
}

/// Autorización por scope: `web::scope("/examenes").wrap(Autorizacion::para(Recurso::Examen))`.
///
/// Debe ir dentro de un scope envuelto por [`AuthMiddleware`]. Deniega (401/403) si faltan los
/// claims, si el método HTTP no corresponde a una [`Accion`] (`HEAD`, `OPTIONS`, ...) o si la
/// política no lo permite; responde 500 si el enforcer no está registrado como `app_data`.
#[derive(Clone, Copy)]
pub struct Autorizacion {
    recurso: Recurso,
}

impl Autorizacion {
    pub fn para(recurso: Recurso) -> Self {
        Self { recurso }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Autorizacion
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AutorizacionService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AutorizacionService {
            service: Rc::new(service),
            recurso: self.recurso,
        })
    }
}

pub struct AutorizacionService<S> {
    service: Rc<S>,
    recurso: Recurso,
}

impl<S, B> Service<ServiceRequest> for AutorizacionService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let recurso = self.recurso;

        Box::pin(async move {
            match autorizar(&req, recurso) {
                Ok(()) => Ok(service.call(req).await?.map_into_left_body()),
                Err(rechazo) => Ok(req.into_response(rechazo.respuesta()).map_into_right_body()),
            }
        })
    }
}

fn autorizar(req: &ServiceRequest, recurso: Recurso) -> Result<(), Rechazo> {
    const DENEGADO: Rechazo = Rechazo::Denegado("Acceso denegado");

    let Some(claims) = req.extensions().get::<Claims>().cloned() else {
        return Err(Rechazo::NoAutenticado("Token no encontrado"));
    };
    let Some(rol) = rol_de(&claims) else {
        return Err(DENEGADO);
    };
    let Ok(accion) = Accion::desde_metodo_http(req.method().as_str()) else {
        warn!(
            "{} {} - metodo sin accion, denegado",
            req.method(),
            req.path()
        );
        return Err(DENEGADO);
    };
    let Some(enforcer) = req.app_data::<web::Data<Enforcer>>() else {
        error!("el enforcer de Casbin no esta registrado como app_data");
        return Err(Rechazo::Interno);
    };

    let solicitud = SolicitudAcceso::new(claims.sub, rol, recurso, accion);
    match CasbinAutorizacion::new(enforcer).verificar_permiso(&solicitud) {
        Ok(()) => {
            debug!(
                "acceso permitido: sub={}, rol={rol}, recurso={recurso}, accion={accion}",
                solicitud.sujeto
            );
            Ok(())
        }
        Err(AutorizacionError::AccesoDenegado) => {
            warn!(
                "acceso denegado: sub={}, rol={rol}, recurso={recurso}, accion={accion}",
                solicitud.sujeto
            );
            Err(DENEGADO)
        }
        Err(e) => {
            error!("no se pudo evaluar la politica RBAC: {e}");
            Err(Rechazo::Interno)
        }
    }
}

fn rol_de(claims: &Claims) -> Option<Rol> {
    claims.rol.as_deref()?.parse().ok()
}

/// Devuelve el token de `Authorization: Bearer <token>`, si lo hay.
pub fn extraer_token(headers: &actix_web::http::header::HeaderMap) -> Option<&str> {
    headers
        .get(actix_web::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

#[cfg(test)]
mod tests {
    use crate::configuration::JwtSettings;
    use crate::controller::auth::casbin_enforcer::{crear_enforcer, permitido_esperado};
    use crate::controller::auth::jwt::{Claims, JWTProvider};
    use crate::startup::configurar_rutas;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::http::{Method, StatusCode};
    use actix_web::{App, test, web};
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use quizz_auth::autorizacion::domain::value_object::accion::Accion;
    use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;
    use quizz_auth::autorizacion::domain::value_object::rol::Rol;
    use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
    use quizz_auth::universal::provider::repositorio::Sesiones;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    const SECRETO: &str = "secreto-de-prueba-con-al-menos-32-bytes";
    const ID: &str = "0b9f1a52-3d4e-4f60-8a7b-9c0d1e2f3a4b";
    const SESION: &str = "sesion-abierta";

    /// Sesiones en memoria: `ID` tiene abierta la sesión `SESION`.
    struct SesionesEnMemoria(Mutex<HashMap<String, String>>);

    impl SesionesEnMemoria {
        fn con_sesion_abierta() -> Arc<dyn Sesiones> {
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
    }

    /// Un almacén de sesiones caído.
    struct SesionesCaidas;

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
    }

    fn token_de_sesion(
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

    fn token_con(rol: &str, secreto: &str, algoritmo: Algorithm, vence_en: i64) -> String {
        token_de_sesion(rol, secreto, algoritmo, vence_en, SESION)
    }

    fn token(rol: Rol) -> String {
        token_con(&rol.to_string(), SECRETO, Algorithm::HS256, 3600)
    }

    async fn app() -> impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
    > {
        app_con(SesionesEnMemoria::con_sesion_abierta()).await
    }

    async fn app_con(
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

    async fn estado(
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

    /// Pasó la autenticación y la autorización (el handler falla después por no tener base de
    /// datos en el test, pero eso ya no es un 401/403).
    fn autorizado(estado: StatusCode) -> bool {
        estado != StatusCode::UNAUTHORIZED && estado != StatusCode::FORBIDDEN
    }

    /// Cada ruta protegida con el recurso y la acción que la política debe aplicar.
    fn rutas_protegidas() -> Vec<(Method, String, Recurso, Accion)> {
        use Accion::*;
        use Recurso::*;
        let get = Method::GET;
        let post = Method::POST;
        let put = Method::PUT;
        let patch = Method::PATCH;
        let delete = Method::DELETE;
        vec![
            (get.clone(), "/examenes".into(), Examen, Leer),
            (post.clone(), format!("/examenes/{ID}"), Examen, Escribir),
            (put.clone(), format!("/examenes/{ID}"), Examen, Actualizar),
            (get.clone(), "/evaluaciones".into(), Evaluacion, Leer),
            (
                post.clone(),
                format!("/evaluaciones/{ID}"),
                Evaluacion,
                Escribir,
            ),
            (
                put.clone(),
                format!("/evaluaciones/{ID}"),
                Evaluacion,
                Actualizar,
            ),
            (
                patch.clone(),
                format!("/evaluaciones/{ID}"),
                Evaluacion,
                Actualizar,
            ),
            (
                post.clone(),
                format!("/evaluaciones/{ID}/respuestas"),
                Evaluacion,
                Escribir,
            ),
            (get.clone(), "/postulantes".into(), Postulante, Leer),
            (put.clone(), "/postulantes".into(), Postulante, Actualizar),
            (
                post.clone(),
                format!("/postulantes/{ID}"),
                Postulante,
                Escribir,
            ),
            (
                delete.clone(),
                format!("/postulantes/{ID}"),
                Postulante,
                Eliminar,
            ),
            (get.clone(), "/psicologos".into(), Psicologo, Leer),
            (
                post.clone(),
                format!("/psicologos/{ID}"),
                Psicologo,
                Escribir,
            ),
            (get.clone(), "/respuestas".into(), Respuesta, Leer),
            (
                get.clone(),
                "/respuestas/asignaciones".into(),
                Respuesta,
                Leer,
            ),
            (get.clone(), format!("/respuestas/{ID}"), Respuesta, Leer),
            (
                patch.clone(),
                format!("/respuestas/{ID}/estado"),
                Respuesta,
                Actualizar,
            ),
            (
                post.clone(),
                format!("/respuestas/{ID}/examenes/{ID}/preguntas/{ID}/contestaciones"),
                Respuesta,
                Escribir,
            ),
            (get.clone(), "/revisiones".into(), Revision, Leer),
            (get.clone(), format!("/revisiones/{ID}"), Revision, Leer),
            (
                post.clone(),
                format!("/revisiones/{ID}"),
                Revision,
                Escribir,
            ),
            (patch, format!("/revisiones/{ID}"), Revision, Actualizar),
            (post, format!("/admins/{ID}"), Admin, Escribir),
        ]
    }

    #[actix_web::test]
    async fn cada_ruta_aplica_la_politica_de_su_recurso_para_cada_rol() {
        let app = app().await;
        for rol in [Rol::Admin, Rol::Psicologo, Rol::Postulante] {
            let token = token(rol);
            for (metodo, uri, recurso, accion) in rutas_protegidas() {
                let estado = estado(&app, metodo.clone(), &uri, Some(&token)).await;
                assert_eq!(
                    autorizado(estado),
                    permitido_esperado(rol, recurso, accion),
                    "{rol} {metodo} {uri} -> {estado}"
                );
            }
        }
    }

    #[actix_web::test]
    async fn sin_token_toda_ruta_protegida_responde_401() {
        let app = app().await;
        for (metodo, uri, _, _) in rutas_protegidas() {
            let estado = estado(&app, metodo.clone(), &uri, None).await;
            assert_eq!(estado, StatusCode::UNAUTHORIZED, "{metodo} {uri}");
        }
    }

    #[actix_web::test]
    async fn una_ruta_codificada_no_salta_la_autorizacion() {
        let app = app().await;
        let postulante = token(Rol::Postulante);
        for uri in [
            format!("/%61dmins/{ID}"),
            format!("/%70sicologos/{ID}"),
            format!("/admins/{ID}"),
            format!("/%65xamenes/{ID}"),
        ] {
            let estado = estado(&app, Method::POST, &uri, Some(&postulante)).await;
            assert_eq!(estado, StatusCode::FORBIDDEN, "{uri}");
        }
    }

    #[actix_web::test]
    async fn los_metodos_sin_accion_se_deniegan() {
        let app = app().await;
        let admin = token(Rol::Admin);
        for metodo in [Method::HEAD, Method::OPTIONS, Method::TRACE] {
            let estado = estado(&app, metodo.clone(), "/examenes", Some(&admin)).await;
            assert_eq!(estado, StatusCode::FORBIDDEN, "{metodo}");
        }
    }

    #[actix_web::test]
    async fn los_tokens_no_validos_responden_401() {
        let app = app().await;
        let rol = Rol::Admin.to_string();
        let otra_clave = token_con(
            &rol,
            "otra-clave-de-al-menos-32-bytes!!",
            Algorithm::HS256,
            3600,
        );
        let expirado = token_con(&rol, SECRETO, Algorithm::HS256, -3600);
        let otro_algoritmo = token_con(&rol, SECRETO, Algorithm::HS512, 3600);
        let sin_firma = format!(
            "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.{}.",
            token(Rol::Admin).split('.').nth(1).unwrap()
        );
        for token in [
            otra_clave,
            expirado,
            otro_algoritmo,
            sin_firma,
            "basura".into(),
        ] {
            let estado = estado(&app, Method::GET, "/examenes", Some(&token)).await;
            assert_eq!(estado, StatusCode::UNAUTHORIZED, "{token}");
        }
    }

    #[actix_web::test]
    async fn un_token_valido_de_una_sesion_cerrada_responde_401() {
        let sesiones = SesionesEnMemoria::con_sesion_abierta();
        let app = app_con(Arc::clone(&sesiones)).await;
        let admin = token(Rol::Admin);
        assert!(autorizado(
            estado(&app, Method::GET, "/examenes", Some(&admin)).await
        ));

        // Equivale a POST /logout, o a un inicio de sesión posterior que la reemplaza.
        sesiones.cerrar(ID, SESION).await.unwrap();

        let estado = estado(&app, Method::GET, "/examenes", Some(&admin)).await;
        assert_eq!(estado, StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn un_token_de_otra_sesion_responde_401() {
        let app = app().await;
        let viejo = token_de_sesion("admin", SECRETO, Algorithm::HS256, 3600, "sesion-anterior");
        let estado = estado(&app, Method::GET, "/examenes", Some(&viejo)).await;
        assert_eq!(estado, StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn si_no_se_puede_consultar_la_sesion_responde_503() {
        let app = app_con(Arc::new(SesionesCaidas)).await;
        let estado = estado(&app, Method::GET, "/examenes", Some(&token(Rol::Admin))).await;
        assert_eq!(estado, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[actix_web::test]
    async fn un_rol_desconocido_se_deniega() {
        let app = app().await;
        let token = token_con("superusuario", SECRETO, Algorithm::HS256, 3600);
        let estado = estado(&app, Method::GET, "/examenes", Some(&token)).await;
        assert_eq!(estado, StatusCode::FORBIDDEN);
    }

    #[actix_web::test]
    async fn las_rutas_publicas_no_piden_token() {
        let app = app().await;
        let estado = estado(&app, Method::GET, "/health-check", None).await;
        assert_eq!(estado, StatusCode::OK);
    }
}
