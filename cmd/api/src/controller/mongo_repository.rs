use actix_web::web;
use mongodb::bson::Document;
use mongodb::{Collection, Database};

/// Acceso a la colección de un adaptador dentro de la base de datos configurada
/// (`database.database_name`).
pub trait MongoRepository {
    fn get_collection_name(&self) -> &str;
    fn get_db(&self) -> &web::Data<Database>;

    fn get_collection(&self) -> Collection<Document> {
        self.get_db()
            .collection::<Document>(self.get_collection_name())
    }
}
