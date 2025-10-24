use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_common::domain::entity::jwt::JwtObject;
use quizz_common::provider::jwt::JwtProviderGenerate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

pub struct JWTProvider {
    secret: String,
    expiration_seconds: i64
}

impl JWTProvider {
    pub fn new(secret: String, expiration_seconds: i64) -> Self {
        Self {
            secret,
            expiration_seconds,
        }
    }
}

#[async_trait]
impl JwtProviderGenerate<PostulanteLoginError> for JWTProvider {
    async fn generar(&self, postulante_id: String) -> Result<JwtObject, PostulanteLoginError> {
        let now = Utc::now().timestamp();
        let expiration = now + self.expiration_seconds;

        let claims = Claims {
            sub: postulante_id,
            exp: expiration,
            iat: now,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| PostulanteLoginError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            value: token,
            expiration: self.expiration_seconds as u32,
        })
    }
}
