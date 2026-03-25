use async_trait::async_trait;

#[async_trait]
pub trait SeguridadPasswordPsicologo<Error>: Send + Sync {
    async fn cifrar(&self, password: String) -> Result<String, Error>;
}
