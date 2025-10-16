use async_trait::async_trait;

#[async_trait]
pub trait RepositorioPostulanteLoginEscritura<Error>: Send + Sync {
    async fn login(&self, usuario: String, password: String) -> Result<(), Error>;
}
