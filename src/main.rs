use std::sync::Arc;

use dashmap::DashMap;
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use tracing::debug;
use tracing_subscriber::prelude::*;

use dolphin_server::{app::app, error::Result, service::Services};

#[tokio::main]
async fn main() -> Result<()> {
    static FILTER: &str = "dolphin=trace,tower_http=debug,axum::rejection=trace";
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| FILTER.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let services = Arc::new(DashMap::new());
    tokio::spawn(async move { Services::browse_services(Arc::clone(&services)).await });

    dotenv().expect(".env file not found");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;

    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:root@localhost:8889/dolphin")
        .await?;

    debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app(pool)).await?;

    Ok(())
}
