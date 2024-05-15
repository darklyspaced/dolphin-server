use axum::{http::StatusCode, routing::get, Router};

async fn check_health() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    Router::new().route("/health", get(check_health))
}
