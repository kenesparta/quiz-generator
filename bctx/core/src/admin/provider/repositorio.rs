use crate::admin::domain::entity::admin::Admin;
use crate::admin::domain::error::admin::AdminError;

pub trait RepositorioAdminEscritura: Send + Sync {
    fn registrar_admin(&self, admin: Admin) -> impl Future<Output = Result<(), AdminError>> + Send;
}
