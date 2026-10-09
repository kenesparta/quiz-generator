//! Conexión con MongoDB y los adaptadores de cada colección (`mongo::<módulo>`), que
//! implementan los puertos del dominio.

pub(crate) mod admin;
pub(crate) mod auth;
pub(crate) mod evaluacion;
pub(crate) mod examen;
pub(crate) mod postulante;
pub(crate) mod pregunta;
pub(crate) mod psicologo;
pub(crate) mod repositorio;
pub(crate) mod respuesta;
pub(crate) mod revision;

use crate::configuration::DatabaseSettings;
use mongodb::bson::doc;
use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;
use std::time::Duration;

/// Conecta con MongoDB y devuelve la base de datos configurada (`database.database_name`).
///
/// Comprueba la conexión con un `ping` antes de devolverla, así que un MongoDB inaccesible
/// falla al arrancar y no en la primera petición. Si la URI no fija los plazos de conexión y de
/// selección de servidor, se usan 5 s y 10 s en lugar de los 10 s y 30 s del driver: con
/// MongoDB caído, cada petición fallaría tras 30 s en vez de responder con un error pronto.
///
/// # Errors
///
/// Falla si la URI no es válida o si MongoDB no responde dentro de esos plazos.
pub async fn create_mongo_client(settings: &DatabaseSettings) -> Result<Database, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(settings.connection_string()).await?;
    client_options.min_pool_size = Some(1);
    client_options
        .connect_timeout
        .get_or_insert(Duration::from_secs(5));
    client_options
        .server_selection_timeout
        .get_or_insert(Duration::from_secs(10));

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;
    tracing::info!("Connected to MongoDB successfully");

    Ok(client.database(&settings.database_name))
}
