mod constantes;
pub mod documento;
pub mod read;
pub mod respuesta_dto;
pub mod write;

use crate::mongo::repositorio::MongoRepository;
use actix_web::web;
use constantes::RESPUESTA_COLLECTION_NAME;

/// El adaptador de la colección `respuesta`, la de las hojas de respuestas. Implementa todos
/// sus puertos: asignación y contestaciones y estado (`write.rs`), lecturas y listados
/// (`read.rs`) y la revisión (`revision/mongo/`).
pub struct RespuestaMongo {
    client: web::Data<mongodb::Database>,
}

impl RespuestaMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RespuestaMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}
