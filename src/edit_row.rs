use askama_axum::IntoResponse;
use axum::{
    body::Bytes,
    debug_handler,
    extract::{Path, State},
};
use serde::Deserialize;

use crate::{app::AppState, config_data::Config, error::Result, row::_row};

#[derive(Deserialize, Debug)]
struct TrolleyForm {
    mac: String,
    device_name: String,
    trolley: String,
}

#[derive(Deserialize, Debug)]
struct ApForm {
    mac: String,
    ap: String,
}

#[debug_handler]
pub async fn edit_row(
    Path((panel, mac)): Path<(String, String)>,
    State(mut state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse> {
    if let Ok(form) = serde_urlencoded::from_bytes::<TrolleyForm>(&body) {
        if form.mac != mac {
            state.trolleys.data.remove(&mac);
        }
        state
            .trolleys
            .data
            .insert(form.mac.clone(), (form.device_name, form.trolley));
        state.trolleys.push_updates(state.pool.clone()).await;
        return _row(panel, form.mac, state).await;
    } else if let Ok(form) = serde_urlencoded::from_bytes::<ApForm>(&body) {
        if form.mac != mac {
            state.ap.data.remove(&mac);
        }
        state.ap.push_updates(state.pool.clone()).await;
        state.ap.data.insert(form.mac.clone(), form.ap);
        return _row(panel, form.mac, state).await;
    }
    unreachable!("failed to parse form as anything");
}
