use actix_web::web;
use mongodb::bson::Document;
use mongodb::{Client, Collection};

pub const MAIN_DATABASE_NAME: &str = "quizz";

pub trait MongoRepository {
    fn get_collection_name(&self) -> &str;
    fn get_client(&self) -> &web::Data<Client>;

    fn get_database_name(&self) -> &str {
        MAIN_DATABASE_NAME
    }

    fn get_collection(&self) -> Collection<Document> {
        self.get_client()
            .database(self.get_database_name())
            .collection::<Document>(self.get_collection_name())
    }
}
