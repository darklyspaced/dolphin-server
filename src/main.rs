use std::env;

use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use tracing::debug;
use tracing_subscriber::prelude::*;

use dolphin_server::{
    app::app, error::Result, load_balancer::LoadBalancer, locations::Locations, service::Services,
};

#[tokio::main]
async fn main() -> Result<()> {
    static FILTER: &str = "dolphin=trace,tower_http=debug,axum::rejection=trace";
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| FILTER.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenv().expect(".env file not found");

    // setup necessary services
    let pool = MySqlPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect(".env should have DATABASE_URL"))
        .await?;
    let mut services = Services::new();
    let locations = Locations::new(pool.clone()).await;

    // start the load balancer
    let balancer = LoadBalancer::new(services.clone(), pool.clone(), locations.clone());
    balancer.run();

    // start observing events
    tokio::spawn(async move { services.browse_services().await });

    // start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app(pool, locations)).await?;

    Ok(())
}
