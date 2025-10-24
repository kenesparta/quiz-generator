#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub application_port: u16,
    pub application_host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

/// Returns the application settings loaded from the configuration file.
///
/// # Returns
///
/// * `Result<Settings, config::ConfigError>` - The parsed settings on success, or a ConfigError on failure
///
/// # Errors
///
/// Returns a ConfigError if:
/// * The configuration file cannot be read
/// * The YAML is invalid
/// * The settings cannot be deserialized into the Settings struct
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
