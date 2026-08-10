mod config;
mod errors;
mod routes;

use anyhow::Result;
use axum::{Router, routing::get};
pub use config::AppConfig;
use tokio::net::TcpListener;

pub async fn serve(config: AppConfig) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = TcpListener::bind(addr).await?;

    let app = Router::new().route("/", get(hello));

    Ok(axum::serve(listener, app).await?)
}

pub async fn hello() -> String {
    format!("hello world")
}
