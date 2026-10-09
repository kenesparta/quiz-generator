use crate::psicologo::domain::entity::psicologo::Psicologo;
use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::psicologo::use_case::listar_psicologos::OutputData;

pub trait RepositorioPsicologoEscritura: Send + Sync {
    fn registrar_psicologo(
        &self,
        psicologo: Psicologo,
    ) -> impl Future<Output = Result<(), PsicologoError>> + Send;

    /// Marca al psicólogo para eliminarlo: desde ese momento no puede iniciar sesión, aunque
    /// el registro sigue ahí. Es idempotente. Si no existe, `RegistroNoEncontrado`.
    fn marcar_eliminado(
        &self,
        id: PsicologoID,
    ) -> impl Future<Output = Result<(), PsicologoError>> + Send;

    /// Borra al psicólogo si está marcado. Es idempotente: si ya no está, no hace nada.
    fn eliminar_psicologo(
        &self,
        id: PsicologoID,
    ) -> impl Future<Output = Result<(), PsicologoError>> + Send;
}

/// Datos públicos del psicólogo (sin password).
pub struct PsicologoInfo {
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub colegiatura: String,
}

pub trait RepositorioPsicologoLectura: Send + Sync {
    fn obtener_psicologo_por_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<PsicologoInfo, PsicologoError>> + Send;
}

pub trait RepositorioPsicologoListar: Send + Sync {
    fn listar_psicologos(
        &self,
    ) -> impl Future<Output = Result<Vec<OutputData>, PsicologoError>> + Send;
}
