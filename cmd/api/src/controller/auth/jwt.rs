use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use quizz_auth::autorizacion::domain::error::autorizacion::AutorizacionError;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_common::domain::entity::jwt::JwtObject;
use quizz_common::provider::jwt::JwtProviderGenerateConRol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub rol: Option<String>,
}

pub struct JWTProvider {
    secret: String,
    expiration_seconds: i64,
}

impl JWTProvider {
    pub fn new(secret: String, expiration_seconds: i64) -> Self {
        Self {
            secret,
            expiration_seconds,
        }
    }

    pub fn verificar_token(&self, token: &str) -> Result<Claims, AutorizacionError> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AutorizacionError::TokenExpirado,
            _ => AutorizacionError::TokenNoValido,
        })?;

        Ok(token_data.claims)
    }
}

#[async_trait]
impl JwtProviderGenerateConRol<LoginUniversalError> for JWTProvider {
    async fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> Result<JwtObject, LoginUniversalError> {
        let now = Utc::now().timestamp();
        let expiration = now + self.expiration_seconds;

        let claims = Claims {
            sub: sujeto_id.clone(),
            exp: expiration,
            iat: now,
            rol: Some(rol.clone()),
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| LoginUniversalError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: sujeto_id,
            value: token,
            expiration: self.expiration_seconds as u64,
            rol: Some(rol),
        })
    }
}
