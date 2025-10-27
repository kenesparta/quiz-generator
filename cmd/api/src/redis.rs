use redis::Client;
use std::error::Error;

pub async fn create_redis_client(connection_string: &str) -> Result<Client, Box<dyn Error>> {
    Ok(Client::open(connection_string)?)
}
