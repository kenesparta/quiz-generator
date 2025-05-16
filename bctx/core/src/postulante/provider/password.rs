use async_trait::async_trait;

#[async_trait]
pub trait SeguridadPassword<E>: Send + Sync {
    async fn cifrar(&self, password: String) -> Result<String, E>;
    async fn comparar(&self, password: String, hashed: String) -> Result<bool, E>;
}
