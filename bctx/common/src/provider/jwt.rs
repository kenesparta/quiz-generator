use crate::domain::entity::jwt::JwtObject;
use async_trait::async_trait;

#[async_trait]
pub trait JwtProviderGenerate<Error>: Send + Sync {
    async fn generar(&self, postulante_id: String) -> Result<JwtObject, Error>;
}

#[async_trait]
pub trait JwtProviderGenerateCertificate<Error>: Send + Sync {
    async fn generar(&self, postulante_id: String, cert: String) -> Result<JwtObject, Error>;
}

#[async_trait]
pub trait JwtProviderVerify<Error>: Send + Sync {
    async fn verificar(&self, jwt_string: String) -> Result<(), Error>;
}
