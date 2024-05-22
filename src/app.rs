use axum::{routing::get, Router};

use crate::{health::check_health, landing::landing, login::login, logout::logout};

pub fn app() -> Router {
    Router::new()
        .route("/health", get(check_health))
        .route("/", get(landing))
        .route("/login", get(login))
        .route("/signout", get(logout))
}
