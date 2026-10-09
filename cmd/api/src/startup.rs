use crate::configuration::JwtSettings;
use crate::controller::admin::route::admin;
use crate::controller::auth::middleware::AuthMiddleware;
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
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use std::net::TcpListener;

/// Construye el enforcer RBAC con el modelo y la política embebidos en el binario.
pub async fn init_casbin_enforcer() -> casbin::Result<casbin::Enforcer> {
    crate::controller::auth::casbin_enforcer::crear_enforcer().await
}

/// Registra todas las rutas de la API.
///
/// Las rutas públicas (`/health-check`, `/login`, `/logout`) quedan fuera del scope
/// autenticado; todo lo demás exige un JWT válido y cada scope declara el recurso que la
/// política RBAC autoriza (ver `controller/auth/middleware.rs`). Requiere como `app_data` el
/// enforcer (`web::Data<casbin::Enforcer>`) además de los clientes que usan los handlers.
pub fn configurar_rutas(cfg: &mut web::ServiceConfig, jwt_secret: &str) {
    cfg.configure(health_check).configure(login_routes).service(
        web::scope("")
            .wrap(AuthMiddleware::new(jwt_secret.to_string()))
            .configure(examen)
            .configure(evaluacion)
            .configure(respuesta)
            .configure(revision)
            .configure(postulante)
            .configure(psicologo)
            .configure(admin),
    );
}

pub fn run(
    tcp_listener: TcpListener,
    mongo_client: MongoClient,
    redis_client: RedisClient,
    jwt_settings: JwtSettings,
    enforcer: casbin::Enforcer,
) -> Result<Server, std::io::Error> {
    let db_connection_pool = web::Data::new(mongo_client);
    let redis_connection_pool = web::Data::new(redis_client);
    let jwt_settings_data = web::Data::new(jwt_settings.clone());
    let enforcer = web::Data::new(enforcer);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(set_cors())
            .configure(|cfg| configurar_rutas(cfg, &jwt_settings.secret))
            .app_data(db_connection_pool.clone())
            .app_data(redis_connection_pool.clone())
            .app_data(jwt_settings_data.clone())
            .app_data(enforcer.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
