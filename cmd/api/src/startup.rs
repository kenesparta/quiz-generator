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
use casbin::{CoreApi, DefaultModel, FileAdapter};
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn init_casbin_enforcer()
-> Result<Arc<RwLock<casbin::Enforcer>>, Box<dyn std::error::Error>> {
    let model = DefaultModel::from_file("rbac/model.conf").await?;
    let adapter = FileAdapter::new("rbac/policy.csv");
    let enforcer = casbin::Enforcer::new(model, adapter).await?;
    Ok(Arc::new(RwLock::new(enforcer)))
}

pub fn run(
    tcp_listener: TcpListener,
    mongo_client: MongoClient,
    redis_client: RedisClient,
    jwt_settings: JwtSettings,
    enforcer: Arc<RwLock<casbin::Enforcer>>,
) -> Result<Server, std::io::Error> {
    let db_connection_pool = web::Data::new(mongo_client);
    let redis_connection_pool = web::Data::new(redis_client);
    let jwt_settings_data = web::Data::new(jwt_settings.clone());
    let server = HttpServer::new(move || {
        let auth_middleware = AuthMiddleware::new(jwt_settings.secret.clone(), enforcer.clone());
        App::new()
            .wrap(set_cors())
            .configure(health_check)
            .configure(login_routes)
            .service(
                web::scope("")
                    .wrap(auth_middleware)
                    .configure(examen)
                    .configure(evaluacion)
                    .configure(respuesta)
                    .configure(revision)
                    .configure(postulante)
                    .configure(psicologo)
                    .configure(admin),
            )
            .app_data(db_connection_pool.clone())
            .app_data(redis_connection_pool.clone())
            .app_data(jwt_settings_data.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
