use crate::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use crate::autorizacion::domain::error::autorizacion::AutorizacionError;
use async_trait::async_trait;

#[async_trait]
pub trait AutorizacionVerificar: Send + Sync {
    async fn verificar_permiso(&self, solicitud: &SolicitudAcceso)
    -> Result<(), AutorizacionError>;
}
