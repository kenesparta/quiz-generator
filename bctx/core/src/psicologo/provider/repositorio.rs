use crate::psicologo::domain::entity::psicologo::Psicologo;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPsicologoEscritura<Error>: Send + Sync {
    async fn registrar_psicologo(&self, psicologo: Psicologo) -> Result<(), Error>;
}

/// Datos públicos del psicólogo (sin password).
pub struct PsicologoInfo {
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub colegiatura: String,
}

#[async_trait]
pub trait RepositorioPsicologoLectura<Error>: Send + Sync {
    async fn obtener_psicologo_por_id(&self, id: String) -> Result<PsicologoInfo, Error>;
}
