use crate::controller::auth::route::postulante_login;
use crate::controller::evaluacion::route::evaluacion;
use crate::controller::examen::route::examen;
use crate::controller::healthcheck::route::health_check;
use crate::controller::postulante::route::postulante;
use crate::controller::respuesta::route::respuesta;
use crate::controller::revision::route::revision;
use crate::cors::set_cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use std::net::TcpListener;

pub fn run(
    tcp_listener: TcpListener,
    mongo_client: MongoClient,
    redis_client: RedisClient,
) -> Result<Server, std::io::Error> {
    let db_connection_pool = web::Data::new(mongo_client);
    let redis_connection_pool = web::Data::new(redis_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(set_cors())
            .configure(postulante)
            .configure(examen)
            .configure(evaluacion)
            .configure(respuesta)
            .configure(revision)
            .configure(postulante_login)
            .configure(health_check)
            .app_data(db_connection_pool.clone())
            .app_data(redis_connection_pool.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
