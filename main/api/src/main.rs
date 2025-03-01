use quizz_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = PgPool::connect("")
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", 3003);
    let tcp_listener = TcpListener::bind(address)?;
    run(tcp_listener, connection_pool)?.await?;
    Ok(())
}
