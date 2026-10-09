use crate::universal::domain::error::login_universal::LoginUniversalError;
use crate::universal::provider::jwt::JwtProviderGenerateConRol;
use crate::universal::provider::repositorio::{RepositorioLoginUniversalLectura, Sesiones};
use quizz_common::domain::value_objects::password_plano::MAX_BYTES_PASSWORD;
use quizz_common::provider::seguridad::Cifrador;
use std::sync::Arc;

pub struct InputData {
    pub documento: String,
    pub password: String,
}

pub struct OutputData {
    pub jwt_value: String,
    pub expiration: u64,
    pub rol: String,
}

/// Inicio de sesión con documento y contraseña para cualquier rol.
///
/// Un documento inexistente y una contraseña incorrecta dan el mismo error y cuestan lo mismo
/// (se hace una verificación ficticia), para que ni la respuesta ni el tiempo revelen qué
/// documentos existen. Una contraseña de más de 72 bytes se rechaza: bcrypt ignoraría el
/// resto y la aceptaría con solo coincidir en los primeros 72.
///
/// Después de abrir la sesión vuelve a buscar la cuenta, porque pudo eliminarse mientras se
/// verificaba la contraseña: si ya no está (o está marcada para eliminarse), cierra la sesión
/// y no entrega el token. Quien elimina una cuenta la marca antes de cerrar su sesión, así que
/// o esta segunda búsqueda ve la marca, o la sesión se abrió antes y la cierra la eliminación.
pub struct LoginUniversal<C, R, J> {
    cifrador: C,
    repositorio: R,
    sesiones: Arc<dyn Sesiones>,
    jwt: J,
}

impl<C: Cifrador, R: RepositorioLoginUniversalLectura, J: JwtProviderGenerateConRol>
    LoginUniversal<C, R, J>
{
    pub fn new(cifrador: C, repositorio: R, sesiones: Arc<dyn Sesiones>, jwt: J) -> Self {
        Self {
            cifrador,
            repositorio,
            sesiones,
            jwt,
        }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<OutputData, LoginUniversalError> {
        if in_.password.len() > MAX_BYTES_PASSWORD {
            return Err(LoginUniversalError::PasswordIncorrecto);
        }

        let usuario = match self
            .repositorio
            .buscar_por_documento(in_.documento.clone())
            .await
        {
            Ok(usuario) => usuario,
            Err(LoginUniversalError::UsuarioNoEncontrado) => {
                self.cifrador.simular_verificacion(in_.password).await;
                return Err(LoginUniversalError::UsuarioNoEncontrado);
            }
            Err(error) => return Err(error),
        };

        if !self
            .cifrador
            .verificar(in_.password, usuario.password)
            .await?
        {
            return Err(LoginUniversalError::PasswordIncorrecto);
        }

        let jwt_object = self
            .jwt
            .generar_con_rol(usuario.id.clone(), usuario.rol.clone())
            .await?;

        self.sesiones
            .abrir(
                &jwt_object.key,
                &jwt_object.sesion_id,
                jwt_object.expiration,
            )
            .await?;

        let sigue_existiendo = match self.repositorio.buscar_por_documento(in_.documento).await {
            Ok(actual) => actual.id == usuario.id && actual.rol == usuario.rol,
            Err(LoginUniversalError::UsuarioNoEncontrado) => false,
            Err(error) => return Err(error),
        };
        if !sigue_existiendo {
            self.sesiones
                .cerrar(&jwt_object.key, &jwt_object.sesion_id)
                .await?;
            return Err(LoginUniversalError::UsuarioNoEncontrado);
        }

        Ok(OutputData {
            jwt_value: jwt_object.value,
            expiration: jwt_object.expiration,
            rol: usuario.rol,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal::domain::usuario_login::UsuarioLogin;
    use async_trait::async_trait;
    use quizz_common::domain::entity::jwt::JwtObject;
    use quizz_common::provider::seguridad::CifradoError;
    use std::sync::{Arc, Mutex};

    const HASH: &str = "hash-de-la-clave-correcta";

    /// Un único usuario, documento 12345678 con la contraseña "clave-correcta".
    struct UnUsuario;

    impl RepositorioLoginUniversalLectura for UnUsuario {
        async fn buscar_por_documento(
            &self,
            documento: String,
        ) -> Result<UsuarioLogin, LoginUniversalError> {
            if documento != "12345678" {
                return Err(LoginUniversalError::UsuarioNoEncontrado);
            }
            Ok(UsuarioLogin {
                id: "usr-1".to_string(),
                password: HASH.to_string(),
                rol: "postulante".to_string(),
            })
        }
    }

    /// El usuario de [`UnUsuario`], que se elimina justo después de la primera búsqueda
    /// (mientras se verifica su contraseña).
    #[derive(Default)]
    struct SeEliminaDuranteElLogin(Mutex<u32>);

    impl RepositorioLoginUniversalLectura for SeEliminaDuranteElLogin {
        async fn buscar_por_documento(
            &self,
            documento: String,
        ) -> Result<UsuarioLogin, LoginUniversalError> {
            let busquedas = {
                let mut busquedas = self.0.lock().unwrap();
                *busquedas += 1;
                *busquedas
            };
            if busquedas > 1 {
                return Err(LoginUniversalError::UsuarioNoEncontrado);
            }
            UnUsuario.buscar_por_documento(documento).await
        }
    }

    /// Anota cada verificación: real (con hash) o ficticia.
    #[derive(Clone, Default)]
    struct CifradorFalso(Arc<Mutex<Vec<&'static str>>>);

    impl Cifrador for CifradorFalso {
        async fn cifrar(&self, _: String) -> Result<String, CifradoError> {
            Ok(HASH.to_string())
        }

        async fn verificar(&self, password: String, hash: String) -> Result<bool, CifradoError> {
            self.0.lock().unwrap().push("real");
            Ok(password == "clave-correcta" && hash == HASH)
        }

        async fn simular_verificacion(&self, _: String) {
            self.0.lock().unwrap().push("ficticia");
        }
    }

    #[derive(Clone, Default)]
    struct SesionesFalsas(Arc<Mutex<Vec<String>>>);

    #[async_trait]
    impl Sesiones for SesionesFalsas {
        async fn abrir(&self, sub: &str, _: &str, _: u64) -> Result<(), LoginUniversalError> {
            self.0.lock().unwrap().push(sub.to_string());
            Ok(())
        }

        async fn es_vigente(&self, _: &str, _: &str) -> Result<bool, LoginUniversalError> {
            Ok(true)
        }

        async fn cerrar(&self, sub: &str, _: &str) -> Result<(), LoginUniversalError> {
            self.0.lock().unwrap().retain(|abierta| abierta != sub);
            Ok(())
        }

        async fn revocar(&self, _: &str) -> Result<(), LoginUniversalError> {
            Ok(())
        }
    }

    struct JwtFalso;

    impl JwtProviderGenerateConRol for JwtFalso {
        async fn generar_con_rol(
            &self,
            sujeto_id: String,
            rol: String,
        ) -> Result<JwtObject, LoginUniversalError> {
            Ok(JwtObject {
                key: sujeto_id,
                value: "token".to_string(),
                sesion_id: "jti".to_string(),
                expiration: 60,
                rol: Some(rol),
            })
        }
    }

    async fn login(
        documento: &str,
        password: &str,
    ) -> (
        Result<OutputData, LoginUniversalError>,
        CifradorFalso,
        SesionesFalsas,
    ) {
        login_con(UnUsuario, documento, password).await
    }

    async fn login_con(
        repositorio: impl RepositorioLoginUniversalLectura,
        documento: &str,
        password: &str,
    ) -> (
        Result<OutputData, LoginUniversalError>,
        CifradorFalso,
        SesionesFalsas,
    ) {
        let (cifrador, sesiones) = (CifradorFalso::default(), SesionesFalsas::default());
        let resultado = LoginUniversal::new(
            cifrador.clone(),
            repositorio,
            Arc::new(sesiones.clone()),
            JwtFalso,
        )
        .ejecutar(InputData {
            documento: documento.to_string(),
            password: password.to_string(),
        })
        .await;
        (resultado, cifrador, sesiones)
    }

    #[tokio::test]
    async fn con_credenciales_correctas_abre_la_sesion() {
        let (resultado, _, sesiones) = login("12345678", "clave-correcta").await;
        assert_eq!(resultado.unwrap().rol, "postulante");
        assert_eq!(*sesiones.0.lock().unwrap(), ["usr-1"]);
    }

    #[tokio::test]
    async fn una_cuenta_eliminada_durante_el_login_no_recibe_token_ni_sesion() {
        let (resultado, cifrador, sesiones) = login_con(
            SeEliminaDuranteElLogin::default(),
            "12345678",
            "clave-correcta",
        )
        .await;
        assert!(matches!(
            resultado,
            Err(LoginUniversalError::UsuarioNoEncontrado)
        ));
        assert_eq!(*cifrador.0.lock().unwrap(), ["real"]);
        assert!(sesiones.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn una_contraseña_incorrecta_no_abre_sesion() {
        let (resultado, cifrador, sesiones) = login("12345678", "otra-clave").await;
        assert!(matches!(
            resultado,
            Err(LoginUniversalError::PasswordIncorrecto)
        ));
        assert_eq!(*cifrador.0.lock().unwrap(), ["real"]);
        assert!(sesiones.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn un_documento_inexistente_cuesta_una_verificacion_como_uno_real() {
        let (resultado, cifrador, _) = login("87654321", "otra-clave").await;
        assert!(matches!(
            resultado,
            Err(LoginUniversalError::UsuarioNoEncontrado)
        ));
        assert_eq!(*cifrador.0.lock().unwrap(), ["ficticia"]);
    }

    #[tokio::test]
    async fn una_contraseña_de_mas_de_72_bytes_se_rechaza() {
        // Coincide con la correcta en los primeros bytes, pero bcrypt no debe ver el resto.
        let larga = format!("clave-correcta{}", "x".repeat(MAX_BYTES_PASSWORD));
        let (resultado, cifrador, _) = login("12345678", &larga).await;
        assert!(matches!(
            resultado,
            Err(LoginUniversalError::PasswordIncorrecto)
        ));
        assert!(cifrador.0.lock().unwrap().is_empty());
    }
}
