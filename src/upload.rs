use askama_axum::IntoResponse;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};

use crate::{app::AppState, config_data::Config, error::Result};

pub async fn upload(
    State(mut state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap().to_vec();
        let string = std::str::from_utf8(&bytes).unwrap();

        state.trolleys.data.clear();

        for line in string.lines() {
            let mut parts = line.split(",");

            state.trolleys.data.insert(
                parts.next().unwrap().to_string(),
                (
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                ),
            );
        }

        state.trolleys.push_updates(state.pool.clone()).await;
    }

    Ok(StatusCode::OK)
}
