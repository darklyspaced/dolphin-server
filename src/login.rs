use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::error::Result;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

pub async fn login_page() -> Result<Html<String>> {
    let page = LoginPage;
    Ok(Html(page.render()?))
}

#[derive(Deserialize)]
struct Details {
    username: String,
    password: String,
}

pub async fn login() -> Result<impl IntoResponse> {
    Ok(StatusCode::OK)
}
