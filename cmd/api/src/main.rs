use quizz_api::cache::crear_conexion_redis;
use quizz_api::configuration::get_configuration;
use quizz_api::indices::crear_indices;
use quizz_api::mongo::create_mongo_client;
use quizz_api::startup::{init_casbin_enforcer, run};
use std::error::Error;
use std::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let configuration = get_configuration()?;
    let database = create_mongo_client(&configuration.database).await?;
    crear_indices(&database).await;
    let redis = crear_conexion_redis(&configuration.redis.connection_string()).await?;
    let enforcer = init_casbin_enforcer().await?;

    let address = format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    );
    let tcp_listener = TcpListener::bind(address)?;
    run(
        tcp_listener,
        database,
        redis,
        &configuration.jwt,
        configuration.cors,
        enforcer,
    )?
    .await?;

    Ok(())
}
