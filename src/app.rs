use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::{
    config::config,
    config_data::{Ap, Trolleys},
    edit_row::edit_row,
    editable_row::editable_row,
    health::check_health,
    landing::landing,
    location::location,
    locations::Locations,
    login::{login, login_page},
    logout::logout,
    ping::ping,
    register::register,
    row::row,
    service::Services,
    upload::upload,
};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub services: Services,
    pub locations: Locations,
    pub ap: Ap,
    pub trolleys: Trolleys,
}

pub fn app(
    pool: MySqlPool,
    locations: Locations,
    services: Services,
    trolleys: Trolleys,
    ap: Ap,
) -> Router {
    let state = AppState {
        pool,
        locations,
        services,
        ap,
        trolleys,
    };

    Router::new()
        .route("/landing", get(landing))
        .route("/health", get(check_health))
        .route("/login", get(login_page).post(login))
        .route("/signout", get(logout))
        .route("/location", post(location))
        .route("/upload", post(upload))
        .route("/ping/:mac", get(ping))
        .route("/register/:mac", get(register))
        .route("/config/:panel", get(config))
        .route("/row/:panel/:mac", get(row).put(edit_row)) // add a put() to edit the row
        .route("/config/row/edit/:panel/:mac", get(editable_row))
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
        .with_state(state)
}
