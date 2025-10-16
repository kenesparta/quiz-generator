use async_trait::async_trait;

#[async_trait]
pub trait JwtProvider<Error>: Send + Sync {
    async fn generar(&self, postulante_id: String) -> Result<String, Error>;
}
