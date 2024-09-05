use askama_axum::Template;
use axum::extract::Path;

use crate::error::Result;

#[derive(Template)]
#[template(path = "config_row.html")]
struct Row {
    panel: String,
    key: String,
    vals: Vec<String>,
}

pub async fn row(Path((panel, mac)): Path<(String, String)>) -> Result<impl IntoResponse> {}
