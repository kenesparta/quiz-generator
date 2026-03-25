use crate::admin::domain::entity::admin::Admin;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioAdminEscritura<Error>: Send + Sync {
    async fn registrar_admin(&self, admin: Admin) -> Result<(), Error>;
}
