use argon2::{Argon2, PasswordHash, PasswordVerifier};
use askama::Template;
use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{AppendHeaders, Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::AppState,
    error::{AuthError, Result},
};

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
    State(state): State<AppState>,
    Form(details): Form<Details>,
    // form must be last as it consumes the request
) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();
    let user = sqlx::query!("SELECT * FROM users WHERE username = ?", details.username)
        .fetch_optional(&pool)
        .await?;

    if let Some(u) = user {
        if u.username == details.username {
            let parsed_hash = PasswordHash::new(&u.username)?;
            let Ok(_) =
                Argon2::default().verify_password(details.password.as_bytes(), &parsed_hash)
            else {
                return Err(AuthError::PasswordIncorrect(details.username).into());
            };

            let session_token = Uuid::new_v4();
            let cookie = AppendHeaders([(SET_COOKIE, format!("session_token={}", session_token))]);

            return Ok((cookie, Redirect::to("/")));
        }
    }

    Err(AuthError::UsernameNotFound(details.username).into())
}
