use anyhow::{Context, Result};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

/// Application configuration root.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: Server,
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_owned(),
            port: 3000,
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
