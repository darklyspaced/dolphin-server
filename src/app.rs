use axum::{extract::MatchedPath, http::Request, routing::get, Router};
use sqlx::MySqlPool;

use crate::{
    health::check_health,
    landing::landing,
    login::{login, login_page},
    logout::logout,
};
use tower_http::trace::TraceLayer;

pub fn app(pool: MySqlPool) -> Router {
    Router::new()
        .route("/health", get(check_health))
        .route("/", get(landing))
        .route("/login", get(login_page).post(login))
        .route("/signout", get(logout))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
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
