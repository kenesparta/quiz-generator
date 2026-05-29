use actix_web::web;
use async_trait::async_trait;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::RepositorioLoginUniversalCacheBorrado;
use redis::AsyncCommands;

pub struct LogoutUniversalRedis {
    client: web::Data<redis::Client>,
}

impl LogoutUniversalRedis {
    pub fn new(client: web::Data<redis::Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RepositorioLoginUniversalCacheBorrado<LoginUniversalError> for LogoutUniversalRedis {
    async fn borrar_token(&self, sujeto_id: String) -> Result<(), LoginUniversalError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("error de redis al conectar: {:?}", e);
                LoginUniversalError::ErrorGenericoCache
            })?;

        let _: () = conn.del(&sujeto_id).await.map_err(|e| {
            log::error!("error de redis al borrar token: {:?}", e);
            LoginUniversalError::ErrorGenericoCache
        })?;

        Ok(())
    }
}
