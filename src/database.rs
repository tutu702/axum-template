use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::any::AnyPoolOptions;
use sqlx::{Any, Pool};
use tracing::info;

use crate::config::DatabaseConfig;

/// A type alias for the application's database connection pool.
pub type DbPool = Pool<Any>;

/// Ensures `sqlx::any::install_default_drivers()` is called exactly once
/// per process.
static DRIVERS_INSTALLED: OnceLock<()> = OnceLock::new();

fn ensure_drivers_installed() {
    DRIVERS_INSTALLED.get_or_init(|| {
        sqlx::any::install_default_drivers();
    });
}

/// Builds a database connection pool from the supplied [`DatabaseConfig`].
///
/// The driver is selected implicitly by the URL scheme (e.g. `sqlite://...`,
/// `postgres://...`), so callers do not need to dispatch on
/// [`DatabaseConfig::driver`] manually.
///
/// # Errors
/// Returns an error if the pool cannot be created (e.g. invalid URL, the
/// database is unreachable, or pool options are rejected).
pub async fn init_pool(cfg: &DatabaseConfig) -> Result<DbPool> {
    ensure_drivers_installed();
    let url = cfg.url.clone();

    let pool = AnyPoolOptions::new()
        .max_connections(cfg.max_connections)
        .min_connections(cfg.min_connections)
        .acquire_timeout(Duration::from_secs(cfg.acquire_timeout))
        .connect(&url)
        .await
        .with_context(|| {
            format!(
                "failed to connect to database (driver: {:?}, url: {})",
                cfg.driver, url
            )
        })?;

    info!(
        driver = ?cfg.driver,
        max_connections = cfg.max_connections,
        min_connections = cfg.min_connections,
        "database pool initialized"
    );

    Ok(pool)
}
