use async_trait::async_trait;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::postulante::provider::repositorio::RepositorioPostulanteCacheEscritura;
use redis::AsyncCommands;

pub struct PostulanteLoginRedis {
    client: redis::Client,
}

impl PostulanteLoginRedis {
    pub fn new(redis_url: &str) -> Result<Self, PostulanteLoginError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| PostulanteLoginError::ErrorGenericoCache)?;

        Ok(Self { client })
    }
}

#[async_trait]
impl RepositorioPostulanteCacheEscritura<PostulanteLoginError> for PostulanteLoginRedis {
    async fn guardar_token(&self, jwt: String) -> Result<(), PostulanteLoginError> {
        let mut con = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| PostulanteLoginError::ErrorGenericoCache)?;

        let _: () = con.set_ex(&jwt, "valid", 86400)
            .await
            .map_err(|e| PostulanteLoginError::ErrorGenericoCache)?;

        Ok(())
    }
}