use crate::usuario::domain::entity::usuario::Usuario;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioUsuarioEscritura<Error>: Send + Sync {
    async fn registrar_usuario(&self, usuario: Usuario) -> Result<(), Error>;
}
