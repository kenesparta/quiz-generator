use quizz_api::configuration::get_configuration;
use quizz_api::mongo::create_mongo_client;
use quizz_api::redis::create_redis_client;
use quizz_api::startup::run;
use std::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = create_mongo_client(&configuration.database.connection_string())
        .await
        .expect("failed to connect to a database");

    let redis_pool = create_redis_client(&configuration.redis.connection_string())
        .await
        .expect("Failed to create Redis client");

    let address = format!(
        "{}:{}",
        configuration.application_host.to_string(),
        configuration.application_port.to_string()
    );

    let tcp_listener = TcpListener::bind(address)?;
    run(tcp_listener, connection_pool, redis_pool)?.await?;

    Ok(())
}
