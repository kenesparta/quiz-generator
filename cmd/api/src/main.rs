use quizz_api::configuration::get_configuration;
use quizz_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", 3003);
    let tcp_listener = TcpListener::bind(address)?;
    run(tcp_listener, connection_pool)?.await?;
    Ok(())
}
