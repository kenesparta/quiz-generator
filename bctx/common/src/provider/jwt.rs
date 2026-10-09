use crate::domain::entity::jwt::JwtObject;
use async_trait::async_trait;

#[async_trait]
pub trait JwtProviderGenerateConRol<Error>: Send + Sync {
    async fn generar_con_rol(&self, sujeto_id: String, rol: String) -> Result<JwtObject, Error>;
}
