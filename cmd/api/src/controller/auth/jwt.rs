use crate::configuration::JwtSettings;
use async_trait::async_trait;
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
    /// Identificador de la sesión: el token solo vale mientras esta sesión siga abierta.
    pub jti: String,
    pub rol: Option<String>,
}

/// Emite y verifica los JWT de sesión (HS256).
///
/// Las claves y la validación se construyen una sola vez al arrancar y se comparten como
/// `web::Data<JWTProvider>`. La verificación solo acepta HS256 (rechaza `alg: none` y otros
/// algoritmos) y exige `exp` y `sub`.
#[derive(Clone)]
pub struct JWTProvider {
    clave_firma: EncodingKey,
    clave_verificacion: DecodingKey,
    validacion: Validation,
    duracion_segundos: u32,
}

impl JWTProvider {
    pub fn new(settings: &JwtSettings) -> Self {
        let mut validacion = Validation::new(Algorithm::HS256);
        validacion.set_required_spec_claims(&["exp", "sub"]);
        Self {
            clave_firma: EncodingKey::from_secret(settings.secret.as_bytes()),
            clave_verificacion: DecodingKey::from_secret(settings.secret.as_bytes()),
            validacion,
            duracion_segundos: settings.expiration_seconds,
        }
    }

    /// Verifica firma, algoritmo y vencimiento, y devuelve los claims.
    ///
    /// # Errors
    ///
    /// [`AutorizacionError::TokenExpirado`] si venció y [`AutorizacionError::TokenNoValido`]
    /// en cualquier otro caso.
    pub fn verificar_token(&self, token: &str) -> Result<Claims, AutorizacionError> {
        decode::<Claims>(token, &self.clave_verificacion, &self.validacion)
            .map(|datos| datos.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AutorizacionError::TokenExpirado
                }
                _ => AutorizacionError::TokenNoValido,
            })
    }
}

#[async_trait]
impl JwtProviderGenerateConRol<LoginUniversalError> for JWTProvider {
    async fn generar_con_rol(
        &self,
        sujeto_id: String,
        rol: String,
    ) -> Result<JwtObject, LoginUniversalError> {
        // `iat` y `exp` son segundos desde la época Unix: no tienen zona horaria.
        let ahora = chrono::Utc::now().timestamp();
        let sesion_id = uuid::Uuid::new_v4().to_string();
        let claims = Claims {
            sub: sujeto_id.clone(),
            exp: ahora + i64::from(self.duracion_segundos),
            iat: ahora,
            jti: sesion_id.clone(),
            rol: Some(rol.clone()),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.clave_firma)
            .map_err(|_| LoginUniversalError::JWTErrorAlGenerar)?;

        Ok(JwtObject {
            key: sujeto_id,
            value: token,
            sesion_id,
            expiration: u64::from(self.duracion_segundos),
            rol: Some(rol),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proveedor(secreto: &str) -> JWTProvider {
        JWTProvider::new(&JwtSettings {
            secret: secreto.to_string(),
            expiration_seconds: 60,
        })
    }

    #[tokio::test]
    async fn un_token_emitido_se_verifica_con_su_sub_y_rol() {
        let jwt = proveedor("un-secreto-de-prueba-de-mas-de-32-bytes");
        let emitido = jwt
            .generar_con_rol("usr-1".to_string(), "admin".to_string())
            .await
            .unwrap();

        let claims = jwt.verificar_token(&emitido.value).unwrap();
        assert_eq!(claims.sub, "usr-1");
        assert_eq!(claims.rol.as_deref(), Some("admin"));
        assert_eq!(claims.exp - claims.iat, 60);
        assert_eq!(claims.jti, emitido.sesion_id);
        assert_eq!(emitido.expiration, 60);
    }

    #[tokio::test]
    async fn un_token_firmado_con_otra_clave_no_es_valido() {
        let emisor = proveedor("un-secreto-de-prueba-de-mas-de-32-bytes");
        let emitido = emisor
            .generar_con_rol("usr-1".to_string(), "admin".to_string())
            .await
            .unwrap();

        let verificador = proveedor("otro-secreto-de-prueba-de-mas-de-32-b");
        assert_eq!(
            verificador.verificar_token(&emitido.value).unwrap_err(),
            AutorizacionError::TokenNoValido
        );
    }

    #[test]
    fn un_token_sin_sub_no_es_valido() {
        #[derive(Serialize)]
        struct SinSub {
            exp: i64,
            rol: String,
        }
        let secreto = "un-secreto-de-prueba-de-mas-de-32-bytes";
        let token = encode(
            &Header::new(Algorithm::HS256),
            &SinSub {
                exp: chrono::Utc::now().timestamp() + 60,
                rol: "admin".to_string(),
            },
            &EncodingKey::from_secret(secreto.as_bytes()),
        )
        .unwrap();

        assert!(proveedor(secreto).verificar_token(&token).is_err());
    }
}
