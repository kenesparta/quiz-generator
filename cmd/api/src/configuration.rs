//! Configuración de la aplicación.
//!
//! Se lee de `configuration.yaml` (opcional) y de variables de entorno con prefijo `QUIZZ`, que
//! tienen prioridad: `QUIZZ_JWT__SECRET`, `QUIZZ_DATABASE__PASSWORD`, `QUIZZ_REDIS__HOST`...
//! En los entornos desplegados los secretos deben venir del entorno, no de un archivo.
//!
//! Los tipos no implementan `Debug` a propósito: contienen secretos.

/// Longitud mínima de la clave HS256 (RFC 7518 §3.2: al menos 256 bits).
pub const LONGITUD_MINIMA_SECRETO_JWT: usize = 32;
const DURACION_MAXIMA_TOKEN_SEGUNDOS: u32 = 7 * 24 * 60 * 60;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub application_port: u16,
    pub application_host: String,
    pub jwt: JwtSettings,
    #[serde(default)]
    pub cors: CorsSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct JwtSettings {
    /// Clave HS256: al menos [`LONGITUD_MINIMA_SECRETO_JWT`] bytes aleatorios
    /// (p. ej. `openssl rand -base64 48`).
    pub secret: String,
    /// Vida del token en segundos: mayor que 0 y como máximo 7 días.
    pub expiration_seconds: u32,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    /// URI completa de MongoDB (`mongodb://` o `mongodb+srv://`, con TLS u otras opciones).
    /// Si está presente se ignoran `host`, `port`, `username` y `password`.
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// URI de conexión. Las credenciales se codifican, así que pueden contener `@`, `:`, `/`...
    pub fn connection_string(&self) -> String {
        if let Some(uri) = &self.uri {
            return uri.clone();
        }
        format!(
            "mongodb://{}{}:{}",
            credenciales(&self.username, &self.password),
            self.host,
            self.port.unwrap_or(27017)
        )
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisSettings {
    /// URI completa de Redis (`redis://` o `rediss://`). Si está presente se ignoran los demás
    /// campos.
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: Option<u16>,
    /// Vacío si Redis no usa ACL (el usuario `default`).
    #[serde(default)]
    pub username: String,
    /// Vacío si Redis no pide contraseña.
    #[serde(default)]
    pub password: String,
}

impl RedisSettings {
    /// URI de conexión. Las credenciales se codifican, así que pueden contener `@`, `:`, `/`...
    pub fn connection_string(&self) -> String {
        if let Some(uri) = &self.uri {
            return uri.clone();
        }
        format!(
            "redis://{}{}:{}",
            credenciales(&self.username, &self.password),
            self.host,
            self.port.unwrap_or(6379)
        )
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct CorsSettings {
    /// Orígenes exactos permitidos, sin `/` final (un navegador nunca lo envía).
    pub allowed_origins: Vec<String>,
}

impl Default for CorsSettings {
    fn default() -> Self {
        Self {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ],
        }
    }
}

/// Lee y valida la configuración (ver la documentación del módulo).
///
/// # Errors
///
/// Falla si falta una clave obligatoria, si un valor no tiene el tipo esperado o si no pasa
/// [`Settings::validar`] (secreto JWT corto, duración del token fuera de rango...).
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    leer_configuracion(variables_de_entorno())
}

fn variables_de_entorno() -> config::Environment {
    config::Environment::with_prefix("QUIZZ")
        .prefix_separator("_")
        .separator("__")
        .list_separator(",")
        .with_list_parse_key("cors.allowed_origins")
        .try_parsing(true)
}

fn leer_configuracion(entorno: config::Environment) -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml).required(false),
        )
        .add_source(entorno)
        .build()?
        .try_deserialize::<Settings>()?
        .validar()
}

impl Settings {
    /// Rechaza configuraciones que arrancarían un servidor inseguro o roto.
    ///
    /// # Errors
    ///
    /// Devuelve un [`config::ConfigError::Message`] que nombra la clave inválida.
    pub fn validar(self) -> Result<Self, config::ConfigError> {
        let error = |mensaje: &str| Err(config::ConfigError::Message(mensaje.to_string()));

        if self.jwt.secret.len() < LONGITUD_MINIMA_SECRETO_JWT {
            return error(
                "jwt.secret: usa al menos 32 bytes aleatorios (p. ej. `openssl rand -base64 48`)",
            );
        }
        if self.jwt.expiration_seconds == 0
            || self.jwt.expiration_seconds > DURACION_MAXIMA_TOKEN_SEGUNDOS
        {
            return error("jwt.expiration_seconds: debe estar entre 1 y 604800 (7 días)");
        }
        if self.database.uri.is_none() && self.database.host.trim().is_empty() {
            return error("database: indica `uri` o `host`");
        }
        if self.redis.uri.is_none() && self.redis.host.trim().is_empty() {
            return error("redis: indica `uri` o `host`");
        }
        if self.cors.allowed_origins.iter().any(|o| o.ends_with('/')) {
            return error("cors.allowed_origins: los orígenes no llevan `/` final");
        }
        Ok(self)
    }
}

/// `usuario:contraseña@` codificados para una URI, o vacío si no hay credenciales.
fn credenciales(usuario: &str, password: &str) -> String {
    if usuario.is_empty() && password.is_empty() {
        return String::new();
    }
    format!("{}:{}@", codificar(usuario), codificar(password))
}

/// Codificación por porcentaje de todo lo que no sea un carácter no reservado (RFC 3986).
fn codificar(valor: &str) -> String {
    let mut codificado = String::with_capacity(valor.len());
    for byte in valor.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            codificado.push(char::from(byte));
        } else {
            codificado.push_str(&format!("%{byte:02X}"));
        }
    }
    codificado
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const SECRETO: &str = "un-secreto-de-prueba-de-mas-de-32-bytes";

    fn entorno(pares: &[(&str, &str)]) -> config::Environment {
        let mapa: HashMap<String, String> = pares
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        variables_de_entorno().source(Some(mapa))
    }

    fn desde_yaml(
        yaml: &str,
        entorno: config::Environment,
    ) -> Result<Settings, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::from_str(yaml, config::FileFormat::Yaml))
            .add_source(entorno)
            .build()?
            .try_deserialize::<Settings>()?
            .validar()
    }

    #[test]
    fn el_ejemplo_de_configuracion_arranca_si_el_entorno_da_el_secreto() {
        let ejemplo = include_str!("../../../configuration.yaml.example");
        let settings = desde_yaml(ejemplo, entorno(&[("QUIZZ_JWT__SECRET", SECRETO)])).unwrap();
        assert_eq!(settings.redis.connection_string(), "redis://127.0.0.1:6379");
        assert_eq!(
            settings.database.connection_string(),
            "mongodb://quizz:quizz@127.0.0.1:27017"
        );
    }

    #[test]
    fn el_ejemplo_sin_secreto_no_arranca() {
        let ejemplo = include_str!("../../../configuration.yaml.example");
        assert!(desde_yaml(ejemplo, entorno(&[])).is_err());
    }

    #[test]
    fn el_entorno_tiene_prioridad_sobre_el_archivo() {
        let ejemplo = include_str!("../../../configuration.yaml.example");
        let settings = desde_yaml(
            ejemplo,
            entorno(&[
                ("QUIZZ_JWT__SECRET", SECRETO),
                ("QUIZZ_APPLICATION_PORT", "9000"),
                ("QUIZZ_DATABASE__DATABASE_NAME", "quizz_staging"),
                (
                    "QUIZZ_CORS__ALLOWED_ORIGINS",
                    "https://app.example.pe,https://admin.example.pe",
                ),
            ]),
        )
        .unwrap();
        assert_eq!(settings.application_port, 9000);
        assert_eq!(settings.database.database_name, "quizz_staging");
        assert_eq!(
            settings.cors.allowed_origins,
            ["https://app.example.pe", "https://admin.example.pe"]
        );
    }

    fn settings_validos() -> Settings {
        let ejemplo = include_str!("../../../configuration.yaml.example");
        desde_yaml(ejemplo, entorno(&[("QUIZZ_JWT__SECRET", SECRETO)])).unwrap()
    }

    #[test]
    fn rechaza_un_secreto_jwt_corto() {
        let mut settings = settings_validos();
        settings.jwt.secret = "a".repeat(LONGITUD_MINIMA_SECRETO_JWT - 1);
        assert!(settings.validar().is_err());

        let mut settings = settings_validos();
        settings.jwt.secret = "a".repeat(LONGITUD_MINIMA_SECRETO_JWT);
        assert!(settings.validar().is_ok());
    }

    #[test]
    fn rechaza_una_duracion_de_token_fuera_de_rango() {
        for duracion in [0, DURACION_MAXIMA_TOKEN_SEGUNDOS + 1] {
            let mut settings = settings_validos();
            settings.jwt.expiration_seconds = duracion;
            assert!(settings.validar().is_err(), "{duracion}");
        }
    }

    #[test]
    fn rechaza_origenes_cors_con_barra_final() {
        let mut settings = settings_validos();
        settings.cors.allowed_origins = vec!["http://localhost:3000/".to_string()];
        assert!(settings.validar().is_err());
    }

    #[test]
    fn las_credenciales_se_codifican_en_la_uri() {
        let mut settings = settings_validos();
        settings.database.username = "app".to_string();
        settings.database.password = "p@ss:w/rd#1".to_string();
        assert_eq!(
            settings.database.connection_string(),
            "mongodb://app:p%40ss%3Aw%2Frd%231@127.0.0.1:27017"
        );
    }

    #[test]
    fn una_uri_explicita_tiene_prioridad() {
        let mut settings = settings_validos();
        settings.database.uri = Some("mongodb+srv://cluster.example.net/?tls=true".to_string());
        assert_eq!(
            settings.database.connection_string(),
            "mongodb+srv://cluster.example.net/?tls=true"
        );
    }
}
