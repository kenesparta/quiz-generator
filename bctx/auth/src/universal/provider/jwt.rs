use crate::universal::domain::error::login_universal::LoginUniversalError;
use quizz_common::domain::entity::jwt::JwtObject;

/// Emite el token de una sesión nueva, con su `jti` y el rol del usuario.
pub trait JwtProviderGenerateConRol: Send + Sync {
    fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> impl Future<Output = Result<JwtObject, LoginUniversalError>> + Send;
}
