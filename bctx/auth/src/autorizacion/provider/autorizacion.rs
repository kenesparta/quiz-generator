use crate::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use crate::autorizacion::domain::error::autorizacion::AutorizacionError;

/// Decide si un rol puede realizar una acción sobre un tipo de recurso.
///
/// Responde a "¿puede el rol R hacer A sobre recursos de tipo K?", nunca a "¿sobre *este*
/// objeto?": la pertenencia (p. ej. que la respuesta sea del postulante) se comprueba en el
/// caso de uso o en el filtro del repositorio.
///
/// # Errors
///
/// [`AutorizacionError::AccesoDenegado`] si la política no lo permite y
/// [`AutorizacionError::ErrorEnforzador`] si la política no se pudo evaluar.
pub trait AutorizacionVerificar {
    fn verificar_permiso(&self, solicitud: &SolicitudAcceso) -> Result<(), AutorizacionError>;
}
