//! Límite de intentos de login, guardado en Redis para que valga entre instancias.
//!
//! Dos contadores con ventana fija: intentos por IP (frena barridos de documentos y el
//! consumo de CPU de bcrypt) y fallos por documento (frena la fuerza bruta sobre una cuenta,
//! sin bloqueo permanente: se libera al cerrar la ventana).
use crate::configuration::LoginSettings;
use actix_web::HttpRequest;
use redis::RedisResult;
use redis::aio::ConnectionManager;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Incrementa y pone la caducidad en un solo paso: si el proceso cae entre ambos, el contador
/// no queda sin caducidad para siempre.
const INCREMENTAR_EN_VENTANA: &str = r#"
local n = redis.call('INCR', KEYS[1])
if n == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[1])
end
return n
"#;

pub struct LimiteDeIntentos {
    conexion: ConnectionManager,
    limites: LoginSettings,
}

impl LimiteDeIntentos {
    pub fn new(conexion: ConnectionManager, limites: LoginSettings) -> Self {
        Self { conexion, limites }
    }

    pub fn ventana_segundos(&self) -> u64 {
        self.limites.ventana_segundos
    }

    /// Cuenta un intento desde la IP del cliente; `false` si ya superó el límite.
    pub async fn permitir_intento(&self, req: &HttpRequest) -> RedisResult<bool> {
        let clave = format!("login:ip:{}", ip_cliente(req, self.limites.detras_de_proxy));
        Ok(self.incrementar(&clave).await? <= self.limites.max_intentos_por_ip)
    }

    /// `true` si el documento acumuló demasiados fallos en la ventana actual.
    pub async fn documento_bloqueado(&self, documento: &str) -> RedisResult<bool> {
        let mut conexion = self.conexion.clone();
        let fallos: Option<u64> = redis::cmd("GET")
            .arg(clave_documento(documento))
            .query_async(&mut conexion)
            .await?;
        Ok(fallos.unwrap_or(0) >= self.limites.max_fallos_por_documento)
    }

    pub async fn registrar_fallo(&self, documento: &str) -> RedisResult<()> {
        self.incrementar(&clave_documento(documento))
            .await
            .map(drop)
    }

    async fn incrementar(&self, clave: &str) -> RedisResult<u64> {
        let mut conexion = self.conexion.clone();
        redis::Script::new(INCREMENTAR_EN_VENTANA)
            .key(clave)
            .arg(self.limites.ventana_segundos)
            .invoke_async(&mut conexion)
            .await
    }
}

/// La clave guarda un resumen del documento, no el DNI.
fn clave_documento(documento: &str) -> String {
    let mut resumen = DefaultHasher::new();
    documento.hash(&mut resumen);
    format!("login:fallos:{:016x}", resumen.finish())
}

/// IP del cliente. Solo detrás de un proxy de confianza se lee `X-Forwarded-For`: si no,
/// cualquiera podría inventarse una IP nueva en cada intento.
fn ip_cliente(req: &HttpRequest, detras_de_proxy: bool) -> String {
    let ip = if detras_de_proxy {
        req.connection_info()
            .realip_remote_addr()
            .map(str::to_string)
    } else {
        req.peer_addr().map(|addr| addr.ip().to_string())
    };
    ip.unwrap_or_else(|| "desconocida".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn sin_proxy_se_ignora_x_forwarded_for() {
        let req = TestRequest::default()
            .peer_addr("10.0.0.7:5000".parse().unwrap())
            .insert_header(("X-Forwarded-For", "1.2.3.4"))
            .to_http_request();
        assert_eq!(ip_cliente(&req, false), "10.0.0.7");
        assert_eq!(ip_cliente(&req, true), "1.2.3.4");
    }

    #[test]
    fn la_clave_del_documento_no_contiene_el_dni() {
        let clave = clave_documento("12345678");
        assert!(!clave.contains("12345678"));
        assert_eq!(clave, clave_documento("12345678"));
        assert_ne!(clave, clave_documento("12345679"));
    }
}
