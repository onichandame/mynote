use tracing::{error, trace};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();
    trace!("server starting");
    gateway::Gateway::create()
        .await?
        .start_server()
        .await
        .map_err(|e| {
            error!("{:?}", e);
            e
        })?;
    Ok(())
}
