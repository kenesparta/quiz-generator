use actix_web::{App, HttpServer};
use quizz_api::controller::applicant::route::applicant;
use quizz_api::controller::healthcheck::route::health_check;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = format!("127.0.0.1:{}", 3003);
    let tcp_listener = TcpListener::bind(address)?;
    let server = HttpServer::new(move || App::new().configure(applicant).configure(health_check))
        .listen(tcp_listener)?
        .run()
        .await?;
    Ok(server)
}
