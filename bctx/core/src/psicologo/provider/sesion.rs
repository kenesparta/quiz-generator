use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::domain::value_object::id::PsicologoID;

/// La sesión de un psicólogo, que vive en el contexto de autenticación (`quizz-auth`).
pub trait SesionPsicologo: Send + Sync {
    /// Cierra la sesión abierta del psicólogo, si la hay: su token deja de autenticar. Es
    /// idempotente. Si no se puede, `SesionNoCerrada`.
    fn cerrar(&self, id: PsicologoID) -> impl Future<Output = Result<(), PsicologoError>> + Send;
}
