use anyhow::Result;
use axum_template::{AppConfig, serve};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    serve(config).await
}