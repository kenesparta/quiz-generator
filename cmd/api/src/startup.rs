use crate::controller::applicant::route::applicant;
use crate::controller::healthcheck::route::health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connection_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .configure(applicant)
            .configure(health_check)
            .app_data(db_connection_pool.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
