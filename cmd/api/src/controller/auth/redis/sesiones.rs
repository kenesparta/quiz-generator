use async_trait::async_trait;
use log::error;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::provider::repositorio::Sesiones;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;

/// Borra la sesión solo si sigue siendo la indicada: comparar y borrar en un único paso evita
/// que el logout de un token viejo cierre la sesión de un inicio de sesión posterior.
const CERRAR_SI_COINCIDE: &str = r#"
if redis.call('GET', KEYS[1]) == ARGV[1] then
    return redis.call('DEL', KEYS[1])
end
return 0
"#;

/// Sesiones en Redis: la clave `sesion:{sujeto_id}` guarda el `jti` vigente y expira con el
/// token. Comparte una única conexión multiplexada (clonar el `ConnectionManager` es barato).
pub struct SesionesRedis {
    conexion: ConnectionManager,
}

impl SesionesRedis {
    pub fn new(conexion: ConnectionManager) -> Self {
        Self { conexion }
    }
}

fn clave(sujeto_id: &str) -> String {
    format!("sesion:{sujeto_id}")
}

fn error_redis(operacion: &'static str) -> impl FnOnce(redis::RedisError) -> LoginUniversalError {
    move |e| {
        error!("error de redis al {operacion} la sesion: {e}");
        LoginUniversalError::ErrorGenericoCache
    }
}

#[async_trait]
impl Sesiones for SesionesRedis {
    async fn abrir(
        &self,
        sujeto_id: &str,
        sesion_id: &str,
        duracion_segundos: u64,
    ) -> Result<(), LoginUniversalError> {
        let mut conexion = self.conexion.clone();
        let _: () = conexion
            .set_ex(clave(sujeto_id), sesion_id, duracion_segundos)
            .await
            .map_err(error_redis("abrir"))?;
        Ok(())
    }

    async fn es_vigente(
        &self,
        sujeto_id: &str,
        sesion_id: &str,
    ) -> Result<bool, LoginUniversalError> {
        let mut conexion = self.conexion.clone();
        let vigente: Option<String> = conexion
            .get(clave(sujeto_id))
            .await
            .map_err(error_redis("consultar"))?;
        Ok(vigente.as_deref() == Some(sesion_id))
    }

    async fn cerrar(&self, sujeto_id: &str, sesion_id: &str) -> Result<(), LoginUniversalError> {
        let mut conexion = self.conexion.clone();
        let _: i64 = redis::Script::new(CERRAR_SI_COINCIDE)
            .key(clave(sujeto_id))
            .arg(sesion_id)
            .invoke_async(&mut conexion)
            .await
            .map_err(error_redis("cerrar"))?;
        Ok(())
    }
}
