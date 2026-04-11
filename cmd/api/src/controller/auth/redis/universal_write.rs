use actix_web::web;
use async_trait::async_trait;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::RepositorioLoginUniversalCacheEscritura;
use quizz_common::domain::entity::jwt::JwtObject;
use redis::AsyncCommands;

pub struct LoginUniversalRedis {
    client: web::Data<redis::Client>,
}

impl LoginUniversalRedis {
    pub fn new(client: web::Data<redis::Client>) -> Result<Self, LoginUniversalError> {
        Ok(Self { client })
    }
}

#[async_trait]
impl RepositorioLoginUniversalCacheEscritura<LoginUniversalError> for LoginUniversalRedis {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), LoginUniversalError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                LoginUniversalError::ErrorGenericoCache
            })?;

        let _: () = conn
            .set_ex(jwt.key, &jwt.value, jwt.expiration)
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                LoginUniversalError::ErrorGenericoCache
            })?;

        Ok(())
    }
}
