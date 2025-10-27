use actix_web::web;
use async_trait::async_trait;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::postulante::provider::repositorio::RepositorioPostulanteCacheEscritura;
use quizz_common::domain::entity::jwt::JwtObject;
use redis::AsyncCommands;

pub struct PostulanteLoginRedis {
    client: web::Data<redis::Client>,
}

impl PostulanteLoginRedis {
    pub fn new(client: web::Data<redis::Client>) -> Result<Self, PostulanteLoginError> {
        Ok(Self { client })
    }
}

#[async_trait]
impl RepositorioPostulanteCacheEscritura<PostulanteLoginError> for PostulanteLoginRedis {
    async fn guardar_token(&self, jwt: JwtObject) -> Result<(), PostulanteLoginError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                println!("{:?}", e);
                PostulanteLoginError::ErrorGenericoCache
            })?;

        let _: () = conn
            .set_ex(jwt.key, &jwt.value, jwt.expiration)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                PostulanteLoginError::ErrorGenericoCache
            })?;

        Ok(())
    }
}
