use axum::response::IntoResponse;
use tracing::info;

use dolphin_server::{app::app, error::Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app()).await?;

    Ok(())
}
