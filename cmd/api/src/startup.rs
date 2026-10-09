use crate::cache::sesiones::SesionesRedis;
use crate::configuration::{CorsSettings, JwtSettings, LoginSettings};
use crate::controller::admin::route::admin;
use crate::controller::auth::jwt::JWTProvider;
use crate::controller::auth::limite_intentos::LimiteDeIntentos;
use crate::controller::auth::middleware::AuthMiddleware;
use crate::controller::auth::route::login_routes;
use crate::controller::error::error_de_extraccion;
use crate::controller::evaluacion::route::evaluacion;
use crate::controller::examen::route::examen;
use crate::controller::healthcheck::route::health_check;
use crate::controller::postulante::route::postulante;
use crate::controller::psicologo::route::psicologo;
use crate::controller::respuesta::route::respuesta;
use crate::controller::revision::route::revision;
use crate::cors::set_cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use mongodb::Database;
use quizz_auth::universal::provider::repositorio::Sesiones;
use redis::aio::ConnectionManager;
use std::net::TcpListener;
use std::sync::Arc;

/// Tamaño máximo de un cuerpo JSON. Las preguntas con imágenes tienen su propio límite en
/// `PUT /examenes/{id}`.
const LIMITE_JSON_BYTES: usize = 2 * 1024 * 1024;

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
    cfg.app_data(
        web::JsonConfig::default()
            .limit(LIMITE_JSON_BYTES)
            .error_handler(error_de_extraccion),
    )
    .app_data(web::QueryConfig::default().error_handler(error_de_extraccion))
    .app_data(web::PathConfig::default().error_handler(error_de_extraccion));

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

/// Una línea por petición: IP, método, ruta sin query string (puede llevar datos personales,
/// como `?documento=`), estado y milisegundos.
fn registro_de_accesos() -> Logger {
    Logger::new(r#"%a "%{metodo}xi %U" %s %D ms"#)
        .custom_request_replace("metodo", |req| req.method().to_string())
}

/// La parte de la configuración que usa el servidor HTTP.
pub struct OpcionesHttp {
    pub jwt: JwtSettings,
    pub cors: CorsSettings,
    pub login: LoginSettings,
}

/// Arranca el servidor HTTP sobre un listener ya abierto (un test puede usar el puerto 0).
pub fn run(
    tcp_listener: TcpListener,
    database: Database,
    redis: ConnectionManager,
    enforcer: casbin::Enforcer,
    opciones: OpcionesHttp,
) -> Result<Server, std::io::Error> {
    let database = web::Data::new(database);
    let sesiones: web::Data<dyn Sesiones> =
        web::Data::from(Arc::new(SesionesRedis::new(redis.clone())) as Arc<dyn Sesiones>);
    let limite_login = web::Data::new(LimiteDeIntentos::new(redis, opciones.login));
    let jwt = web::Data::new(JWTProvider::new(&opciones.jwt));
    let enforcer = web::Data::new(enforcer);
    let cors = opciones.cors;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(set_cors(&cors.allowed_origins))
            .wrap(registro_de_accesos())
            .configure(configurar_rutas)
            .app_data(database.clone())
            .app_data(sesiones.clone())
            .app_data(limite_login.clone())
            .app_data(jwt.clone())
            .app_data(enforcer.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
