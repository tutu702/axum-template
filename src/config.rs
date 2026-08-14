use anyhow::{Context, Result};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

/// Application configuration root.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server: Server,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 3000,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
    pub secret: String,
    #[serde(rename = "expireMinutes")]
    pub expire_minutes: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            username: "admin".into(),
            password: "123456".into(),
            secret: "axum-template".into(),
            expire_minutes: 30,
        }
    }
}

impl AppConfig {
    /// Loads configuration by merging (lowest → highest precedence):
    /// 1. Built-in defaults.
    /// 2. `config.yaml` (optional).
    /// 3. `APP_*` environment variables.
    ///
    /// A `.env` file is loaded best-effort into the process environment
    /// before the sources above are read.
    ///
    /// # Errors
    /// Returns an error if the configuration sources cannot be built or
    /// the merged values cannot be deserialized into [`AppConfig`].
    pub fn load() -> Result<Self> {
        // The `.env` file is optional; its absence should not block startup.
        let _ = dotenvy::dotenv();

        Config::builder()
            .add_source(
                File::with_name("config")
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                Environment::with_prefix("APP")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()
            .context("failed to build configuration sources")?
            .try_deserialize()
            .context("failed to deserialize AppConfig; check field types and required entries")
    }
}
