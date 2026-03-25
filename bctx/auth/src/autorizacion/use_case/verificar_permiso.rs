use crate::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use crate::autorizacion::domain::error::autorizacion::AutorizacionError;
use crate::autorizacion::provider::autorizacion::AutorizacionVerificar;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct VerificarPermiso {
    autorizacion: Box<dyn AutorizacionVerificar>,
}

impl VerificarPermiso {
    pub fn new(autorizacion: Box<dyn AutorizacionVerificar>) -> Self {
        Self { autorizacion }
    }
}

#[async_trait]
impl CasoDeUso<SolicitudAcceso, (), AutorizacionError> for VerificarPermiso {
    async fn ejecutar(&self, solicitud: SolicitudAcceso) -> Result<(), AutorizacionError> {
        self.autorizacion.verificar_permiso(&solicitud).await
    }
}
