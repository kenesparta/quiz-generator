use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use quizz_auth::admin::domain::error::admin::AdminLoginError;
use quizz_auth::autorizacion::domain::error::autorizacion::AutorizacionError;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::psicologo::domain::error::psicologo::PsicologoLoginError;
use quizz_common::domain::entity::jwt::JwtObject;
use quizz_common::provider::jwt::{JwtProviderGenerate, JwtProviderGenerateConRol};
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
impl JwtProviderGenerate<PostulanteLoginError> for JWTProvider {
    async fn generar(&self, postulante_id: String) -> Result<JwtObject, PostulanteLoginError> {
        let now = Utc::now().timestamp();
        let expiration = now + self.expiration_seconds;

        let claims = Claims {
            sub: postulante_id.clone(),
            exp: expiration,
            iat: now,
            rol: Some("postulante".to_string()),
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| PostulanteLoginError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: postulante_id,
            value: token,
            expiration: self.expiration_seconds as u64,
            rol: Some("postulante".to_string()),
        })
    }
}

#[async_trait]
impl JwtProviderGenerateConRol<PostulanteLoginError> for JWTProvider {
    async fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> Result<JwtObject, PostulanteLoginError> {
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
        .map_err(|_| PostulanteLoginError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: sujeto_id,
            value: token,
            expiration: self.expiration_seconds as u64,
            rol: Some(rol),
        })
    }
}

#[async_trait]
impl JwtProviderGenerateConRol<PsicologoLoginError> for JWTProvider {
    async fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> Result<JwtObject, PsicologoLoginError> {
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
        .map_err(|_| PsicologoLoginError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: sujeto_id,
            value: token,
            expiration: self.expiration_seconds as u64,
            rol: Some(rol),
        })
    }
}

#[async_trait]
impl JwtProviderGenerateConRol<AdminLoginError> for JWTProvider {
    async fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> Result<JwtObject, AdminLoginError> {
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
        .map_err(|_| AdminLoginError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: sujeto_id,
            value: token,
            expiration: self.expiration_seconds as u64,
            rol: Some(rol),
        })
    }
}
