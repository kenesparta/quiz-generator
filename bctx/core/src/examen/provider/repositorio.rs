use crate::examen::domain::entity::examen::Examen;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioExamenEscritura<Error>: Send + Sync {
    async fn guardar_examen(&self, examen: Examen) -> Result<(), Error>;
}

#[async_trait]
pub trait RepositorioExamenLectura<Error>: Send + Sync {
    async fn obtener_examen(&self, id: &str) -> Result<Examen, Error>;
}
