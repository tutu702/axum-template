mod config;
mod errors;
pub mod handler;
pub mod models;
pub mod routes;

use anyhow::Result;
pub use config::AppConfig;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::routes::init_routes;

pub async fn serve(config: AppConfig) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = TcpListener::bind(addr.clone()).await?;

    let app = init_routes()?;

    tracing::info!("listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}
