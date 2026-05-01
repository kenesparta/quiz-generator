use crate::examen::domain::entity::examen::Examen;
use crate::examen::use_case::listar_examenes::OutputData;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioExamenEscritura<Error>: Send + Sync {
    async fn guardar_examen(&self, examen: Examen) -> Result<(), Error>;
}

#[async_trait]
pub trait RepositorioExamenLectura<Error>: Send + Sync {
    async fn obtener_examen(&self, id: &str) -> Result<Examen, Error>;
}

#[async_trait]
pub trait RepositorioExamenListar<Error>: Send + Sync {
    async fn listar_examenes(&self) -> Result<Vec<OutputData>, Error>;
}
