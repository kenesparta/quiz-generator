use mongodb::bson::doc;
use mongodb::{Client, options::ClientOptions};
use std::error::Error;

pub async fn create_mongo_client(connection_string: &str) -> Result<Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(connection_string).await?;
    client_options.max_pool_size = Some(10);
    client_options.min_pool_size = Some(1);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;
    println!("Connected to MongoDB successfully");

    Ok(client)
}
