use actix_web::web;
use async_trait::async_trait;
use quizz_auth::admin::domain::error::admin::AdminLoginError;
use quizz_auth::admin::provider::repositorio::RepositorioAdminCacheEscritura;
use quizz_common::domain::entity::jwt::JwtObject;
use redis::AsyncCommands;

pub struct AdminLoginRedis {
    client: web::Data<redis::Client>,
}

impl AdminLoginRedis {
    pub fn new(client: web::Data<redis::Client>) -> Result<Self, AdminLoginError> {
        Ok(Self { client })
    }
}

#[async_trait]
impl RepositorioAdminCacheEscritura<AdminLoginError> for AdminLoginRedis {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), AdminLoginError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                AdminLoginError::ErrorGenericoCache
            })?;

        let _: () = conn
            .set_ex(jwt.key, &jwt.value, jwt.expiration)
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                AdminLoginError::ErrorGenericoCache
            })?;

        Ok(())
    }
}
