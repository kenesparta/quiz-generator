use async_trait::async_trait;

#[async_trait]
pub trait SeguridadPasswordAdmin<Error>: Send + Sync {
    async fn cifrar(&self, password: String) -> Result<String, Error>;
}
