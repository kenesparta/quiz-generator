use crate::psicologo::domain::entity::psicologo::Psicologo;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPsicologoEscritura<Error>: Send + Sync {
    async fn registrar_psicologo(&self, psicologo: Psicologo) -> Result<(), Error>;
}
