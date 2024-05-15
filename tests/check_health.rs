use anyhow::Result;
use axum::{body::Body, http::Request};
use tower::ServiceExt;

use dolphin_server::app;

#[tokio::test]
async fn check_health_works() -> Result<()> {
    let response = app()
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;

    assert!(response.status().is_success());
    Ok(())
}
