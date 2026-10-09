use crate::universal::domain::error::login_universal::LoginUniversalError;
use crate::universal::domain::usuario_login::UsuarioLogin;
use async_trait::async_trait;

/// Busca la cuenta de un documento en las colecciones de admins, psicólogos y postulantes.
pub trait RepositorioLoginUniversalLectura: Send + Sync {
    fn buscar_por_documento(
        &self,
        documento: String,
    ) -> impl Future<Output = Result<UsuarioLogin, LoginUniversalError>> + Send;
}

/// Sesiones abiertas: como mucho una por usuario, identificada por el `jti` de su token.
///
/// El token solo es válido mientras su sesión siga abierta: la autenticación consulta
/// [`Sesiones::es_vigente`] en cada petición, así que cerrar la sesión revoca el token aunque
/// su firma y su `exp` sigan siendo válidos. Se usa como `dyn` (Redis en producción, memoria
/// en los tests).
#[async_trait]
pub trait Sesiones: Send + Sync {
    /// Abre la sesión `sesion_id` de `sujeto_id` por `duracion_segundos`. Reemplaza la sesión
    /// anterior: iniciar sesión de nuevo invalida el token previo.
    async fn abrir(
        &self,
        sujeto_id: &str,
        sesion_id: &str,
        duracion_segundos: u64,
    ) -> Result<(), LoginUniversalError>;

    /// `true` si `sesion_id` es la sesión abierta de `sujeto_id`.
    async fn es_vigente(
        &self,
        sujeto_id: &str,
        sesion_id: &str,
    ) -> Result<bool, LoginUniversalError>;

    /// Cierra la sesión si `sesion_id` sigue siendo la abierta. Es idempotente: si ya se cerró
    /// o la reemplazó un inicio de sesión posterior, no hace nada.
    async fn cerrar(&self, sujeto_id: &str, sesion_id: &str) -> Result<(), LoginUniversalError>;

    /// Cierra la sesión abierta de `sujeto_id`, sea cual sea su `sesion_id`: se usa cuando se
    /// elimina la cuenta. Es idempotente.
    async fn revocar(&self, sujeto_id: &str) -> Result<(), LoginUniversalError>;
}
