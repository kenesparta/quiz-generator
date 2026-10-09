use actix_web::web;
use mongodb::bson::Document;
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::{Collection, Database};

/// Campo que marca una cuenta mientras se elimina (ver `EliminarPsicologo`): con él ya no
/// puede iniciar sesión, aunque el documento sigue ahí hasta que se cierra su sesión.
pub const CAMPO_ELIMINADO: &str = "eliminado";

/// `true` si la escritura violó un índice único (error E11000 de MongoDB).
pub fn es_clave_duplicada(error: &mongodb::error::Error) -> bool {
    matches!(
        error.kind.as_ref(),
        ErrorKind::Write(WriteFailure::WriteError(e)) if e.code == 11000
    )
}

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
