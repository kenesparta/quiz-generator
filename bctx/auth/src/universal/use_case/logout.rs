use crate::universal::domain::error::login_universal::LoginUniversalError;
use crate::universal::provider::repositorio::Sesiones;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::sync::Arc;

pub struct InputData {
    pub sujeto_id: String,
    /// `jti` del token con el que se pide cerrar la sesión.
    pub sesion_id: String,
}

/// Cierra la sesión del token: desde ese momento el token deja de autenticar.
pub struct Logout {
    sesiones: Arc<dyn Sesiones>,
}

impl Logout {
    pub fn new(sesiones: Arc<dyn Sesiones>) -> Logout {
        Self { sesiones }
    }
}

#[async_trait]
impl CasoDeUso<InputData, (), LoginUniversalError> for Logout {
    async fn ejecutar(&self, in_: InputData) -> Result<(), LoginUniversalError> {
        self.sesiones.cerrar(&in_.sujeto_id, &in_.sesion_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct SesionesFalsas {
        cerradas: Arc<Mutex<Vec<(String, String)>>>,
    }

    #[async_trait]
    impl Sesiones for SesionesFalsas {
        async fn abrir(&self, _: &str, _: &str, _: u64) -> Result<(), LoginUniversalError> {
            Ok(())
        }

        async fn es_vigente(&self, _: &str, _: &str) -> Result<bool, LoginUniversalError> {
            Ok(true)
        }

        async fn cerrar(
            &self,
            sujeto_id: &str,
            sesion_id: &str,
        ) -> Result<(), LoginUniversalError> {
            self.cerradas
                .lock()
                .unwrap()
                .push((sujeto_id.to_string(), sesion_id.to_string()));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_logout_cierra_la_sesion_del_token() {
        let cerradas = Arc::new(Mutex::new(Vec::new()));
        let use_case = Logout::new(Arc::new(SesionesFalsas {
            cerradas: Arc::clone(&cerradas),
        }));

        use_case
            .ejecutar(InputData {
                sujeto_id: "usr-123".to_string(),
                sesion_id: "jti-1".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(
            *cerradas.lock().unwrap(),
            [("usr-123".to_string(), "jti-1".to_string())]
        );
    }
}
