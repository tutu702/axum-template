mod config;
mod errors;
pub mod handler;
pub mod models;
pub mod routes;
mod state;

use anyhow::{Context, Result};
pub use config::AppConfig;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{routes::routes::init_routes, state::AppState};

pub async fn serve(config: AppConfig) -> Result<()> {
    init_tracing();

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind to {addr}"))?;

    let state = AppState::new(config);
    let app = init_routes(state);

    tracing::info!("listening on {addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .context("axum::serve returned an error")?;

    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();
}