use redis::RedisResult;
use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use std::time::Duration;

/// Abre UNA conexión a Redis compartida por toda la aplicación.
///
/// Clonarla es barato (un `Arc`) y los comandos concurrentes se multiplexan sobre el mismo
/// socket; tras un reinicio de Redis se reconecta sola (el primer comando tras la caída falla
/// mientras reconecta). Se conecta al llamarla, así que un Redis inaccesible impide arrancar
/// en lugar de fallar en el primer login. Los plazos se fijan aquí para que una actualización
/// del crate no los cambie sin aviso.
///
/// # Errors
///
/// Falla si la URI no es válida o si Redis no responde tras los reintentos.
pub async fn crear_conexion_redis(url: &str) -> RedisResult<ConnectionManager> {
    let client = redis::Client::open(url)?;
    let config = ConnectionManagerConfig::new()
        .set_connection_timeout(Some(Duration::from_secs(1)))
        .set_response_timeout(Some(Duration::from_millis(500)));
    ConnectionManager::new_with_config(client, config).await
}
