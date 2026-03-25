use actix_web::web;
use async_trait::async_trait;
use quizz_auth::psicologo::domain::error::psicologo::PsicologoLoginError;
use quizz_auth::psicologo::provider::repositorio::RepositorioPsicologoCacheEscritura;
use quizz_common::domain::entity::jwt::JwtObject;
use redis::AsyncCommands;

pub struct PsicologoLoginRedis {
    client: web::Data<redis::Client>,
}

impl PsicologoLoginRedis {
    pub fn new(client: web::Data<redis::Client>) -> Result<Self, PsicologoLoginError> {
        Ok(Self { client })
    }
}

#[async_trait]
impl RepositorioPsicologoCacheEscritura<PsicologoLoginError> for PsicologoLoginRedis {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), PsicologoLoginError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                PsicologoLoginError::ErrorGenericoCache
            })?;

        let _: () = conn
            .set_ex(jwt.key, &jwt.value, jwt.expiration)
            .await
            .map_err(|e| {
                log::error!("error de redis: {:?}", e);
                PsicologoLoginError::ErrorGenericoCache
            })?;

        Ok(())
    }
}
