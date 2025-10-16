use async_trait::async_trait;

#[async_trait]
pub trait SeguridadCifrar<Error>: Send + Sync {
    async fn cifrar(&self, password: String) -> Result<String, Error>;
}

#[async_trait]
pub trait SeguridadComparar<Error>: Send + Sync {
    async fn comparar(&self, password: String, hashed: String) -> Result<bool, Error>;
}
