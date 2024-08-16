use crate::error::Result;
use askama_axum::IntoResponse;
use axum::extract::Path;

pub async fn config(Path(panel): Path<String>) -> Result<impl IntoResponse> {
    match &panel {
        "" =>
        _ => Err
    }
}
