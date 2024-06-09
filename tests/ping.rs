use anyhow::Result;
use axum::{body::Body, http::Request};
use sqlx::mysql::MySqlPoolOptions;
use tower::util::ServiceExt;

use dolphin_server::app::app;

#[tokio::test]
async fn ping_works() -> Result<()> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:root@localhost:8889/dolphin")
        .await?;

    let response = app(pool)
        .oneshot(Request::builder().uri("/ping/").body(Body::empty())?)
        .await?;

    assert!(response.status().is_success());
    Ok(())
}
