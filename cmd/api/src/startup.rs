use crate::configuration::{CorsSettings, JwtSettings};
use crate::controller::admin::route::admin;
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::middleware::AuthMiddleware;
use crate::controller::auth::redis::sesiones::SesionesRedis;
use crate::controller::auth::route::login_routes;
use crate::controller::evaluacion::route::evaluacion;
use crate::controller::examen::route::examen;
use crate::controller::healthcheck::route::health_check;
use crate::controller::postulante::route::postulante;
use crate::controller::psicologo::route::psicologo;
use crate::controller::respuesta::route::respuesta;
use crate::controller::revision::route::revision;
use crate::cors::set_cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use mongodb::Database;
use quizz_auth::universal::provider::repositorio::Sesiones;
use redis::aio::ConnectionManager;
use std::net::TcpListener;
use std::sync::Arc;

/// Construye el enforcer RBAC con el modelo y la política embebidos en el binario.
pub async fn init_casbin_enforcer() -> casbin::Result<casbin::Enforcer> {
    crate::controller::auth::casbin_enforcer::crear_enforcer().await
}

/// Registra todas las rutas de la API.
///
/// Las rutas públicas (`/health-check`, `/login`, `/logout`) quedan fuera del scope
/// autenticado; todo lo demás exige un JWT válido y cada scope declara el recurso que la
/// política RBAC autoriza (ver `controller/auth/middleware.rs`). Requiere como `app_data` el
/// enforcer (`web::Data<casbin::Enforcer>`), el proveedor JWT (`web::Data<JWTProvider>`) y
/// las sesiones (`web::Data<dyn Sesiones>`), además de los clientes que usan los handlers.
pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.configure(health_check).configure(login_routes).service(
        web::scope("")
            .wrap(AuthMiddleware)
            .configure(examen)
            .configure(evaluacion)
            .configure(respuesta)
            .configure(revision)
            .configure(postulante)
            .configure(psicologo)
            .configure(admin),
    );
}

/// Arranca el servidor HTTP sobre un listener ya abierto (un test puede usar el puerto 0).
pub fn run(
    tcp_listener: TcpListener,
    database: Database,
    redis: ConnectionManager,
    jwt_settings: &JwtSettings,
    cors_settings: CorsSettings,
    enforcer: casbin::Enforcer,
) -> Result<Server, std::io::Error> {
    let database = web::Data::new(database);
    let sesiones: web::Data<dyn Sesiones> =
        web::Data::from(Arc::new(SesionesRedis::new(redis)) as Arc<dyn Sesiones>);
    let jwt = web::Data::new(JWTProvider::new(jwt_settings));
    let enforcer = web::Data::new(enforcer);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(set_cors(&cors_settings.allowed_origins))
            .configure(configurar_rutas)
            .app_data(database.clone())
            .app_data(sesiones.clone())
            .app_data(jwt.clone())
            .app_data(enforcer.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
