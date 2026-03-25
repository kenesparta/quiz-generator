use async_trait::async_trait;
use casbin::{CoreApi, Enforcer};
use quizz_auth::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use quizz_auth::autorizacion::domain::error::autorizacion::AutorizacionError;
use quizz_auth::autorizacion::provider::autorizacion::AutorizacionVerificar;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CasbinAutorizacion {
    enforcer: Arc<RwLock<Enforcer>>,
}

impl CasbinAutorizacion {
    pub fn new(enforcer: Arc<RwLock<Enforcer>>) -> Self {
        Self { enforcer }
    }
}

#[async_trait]
impl AutorizacionVerificar for CasbinAutorizacion {
    async fn verificar_permiso(
        &self,
        solicitud: &SolicitudAcceso,
    ) -> Result<(), AutorizacionError> {
        let enforcer = self.enforcer.read().await;

        let rol = solicitud.rol.to_string();
        let recurso = solicitud.recurso.to_string();
        let accion = solicitud.accion.to_string();

        let permitido = enforcer
            .enforce(vec![rol, recurso, accion])
            .map_err(|_| AutorizacionError::ErrorEnforzador)?;

        if permitido {
            Ok(())
        } else {
            Err(AutorizacionError::AccesoDenegado)
        }
    }
}
