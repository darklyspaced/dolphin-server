use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use tracing::info;

use crate::error::Result;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

/// Displays login page to user
pub async fn login_page() -> Result<Html<String>> {
    let page = LoginPage;
    Ok(Html(page.render()?))
}

#[derive(Deserialize)]
/// Representation of the structure of the login page form
pub struct Details {
    username: String,
    password: String,
}

/// Handles login request from user
pub async fn login(
    State(pool): State<MySqlPool>,
    Form(details): Form<Details>,
    // form must be last as it consumes the request
) -> Result<impl IntoResponse> {
    let users = sqlx::query!("SELECT * FROM users").fetch_all(&pool).await?;

    for user in users {
        info!("user found: {}", user.email);
    }

    info!(
        "the user's details are: {} and {}",
        details.username, details.password
    );

    Ok(StatusCode::OK)
}
