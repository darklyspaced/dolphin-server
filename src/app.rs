use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::{
    health::check_health,
    landing::landing,
    location::location,
    login::{login, login_page},
    logout::logout,
};
use tower_http::trace::TraceLayer;

pub fn app(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/health", get(check_health))
        .route("/login", get(login_page).post(login))
        .route("/signout", get(logout))
        .route("/location", post(location))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(pool)
}
