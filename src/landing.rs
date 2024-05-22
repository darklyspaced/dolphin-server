use axum::{http::HeaderMap, response::Redirect};
use tracing::info;

pub async fn landing(_headers: HeaderMap) -> Redirect {
    info!("on landing page");
    Redirect::permanent("/health")
}
