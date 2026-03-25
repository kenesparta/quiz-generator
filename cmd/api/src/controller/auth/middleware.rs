use crate::controller::auth::casbin_enforcer::CasbinAutorizacion;
use crate::controller::auth::jwt::JWTProvider;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use casbin::Enforcer;
use futures::future::{LocalBoxFuture, Ready, ok};
use quizz_auth::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use quizz_auth::autorizacion::domain::value_object::accion::Accion;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_auth::autorizacion::provider::autorizacion::AutorizacionVerificar;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AuthMiddleware {
    jwt_secret: String,
    enforcer: Arc<RwLock<Enforcer>>,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String, enforcer: Arc<RwLock<Enforcer>>) -> Self {
        Self {
            jwt_secret,
            enforcer,
        }
    }
}

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
            service: Arc::new(service),
            jwt_secret: self.jwt_secret.clone(),
            enforcer: self.enforcer.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Arc<S>,
    jwt_secret: String,
    enforcer: Arc<RwLock<Enforcer>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();
        let enforcer = self.enforcer.clone();

        Box::pin(async move {
            // Extraer token del header Authorization
            let token = match extraer_token(&req) {
                Some(t) => t,
                None => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": "Token no encontrado"}));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // Verificar y decodificar JWT
            let jwt_provider = JWTProvider::new(jwt_secret, 0);
            let claims = match jwt_provider.verificar_token(&token) {
                Ok(c) => c,
                Err(_) => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": "Token no valido o expirado"}));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // Extraer rol del token
            let rol_str = match &claims.rol {
                Some(r) => r.clone(),
                None => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": "Rol no encontrado en el token"}));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            let rol = match rol_str.parse::<Rol>() {
                Ok(r) => r,
                Err(_) => {
                    let response = HttpResponse::Forbidden()
                        .json(serde_json::json!({"error": "Rol no valido"}));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // Determinar recurso desde la ruta
            let ruta = req.path().to_string();
            let recurso = match Recurso::desde_ruta(&ruta) {
                Ok(r) => r,
                Err(_) => {
                    // Si no es un recurso protegido, dejar pasar
                    let res = service.call(req).await?;
                    return Ok(res.map_into_left_body());
                }
            };

            // Determinar accion desde el metodo HTTP
            let metodo = req.method().as_str().to_string();
            let accion = match Accion::desde_metodo_http(&metodo) {
                Ok(a) => a,
                Err(_) => {
                    let res = service.call(req).await?;
                    return Ok(res.map_into_left_body());
                }
            };

            // Verificar permiso con casbin
            let solicitud = SolicitudAcceso::new(claims.sub.clone(), rol, recurso, accion);
            let autorizacion = CasbinAutorizacion::new(enforcer);

            match autorizacion.verificar_permiso(&solicitud).await {
                Ok(()) => {
                    req.extensions_mut().insert(claims);
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(_) => {
                    let response = HttpResponse::Forbidden()
                        .json(serde_json::json!({"error": "Acceso denegado"}));
                    Ok(req.into_response(response).map_into_right_body())
                }
            }
        })
    }
}

fn extraer_token(req: &ServiceRequest) -> Option<String> {
    let auth_header = req.headers().get("Authorization")?.to_str().ok()?;

    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}
