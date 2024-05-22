use anyhow::Result;
use axum::{body::Body, http::Request};
use tower::ServiceExt;

use dolphin_server::app::app;

#[tokio::test]
async fn landing_redirect() -> Result<()> {
    let request = Request::builder().uri("/").body(Body::empty())?;

    let response = app().oneshot(request).await?;

    assert!(response.status().is_redirection());
    Ok(())
}
