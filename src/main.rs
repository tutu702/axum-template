use axum_template::{AppConfig, serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()?;

    if let Err(e) = serve(config).await {
        panic!("Failed to start application: {:?}", e);
    }

    Ok(())
}
