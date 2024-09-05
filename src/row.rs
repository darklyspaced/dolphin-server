use askama_axum::{IntoResponse, Template};
use axum::extract::{Path, State};

use crate::{
    app::AppState,
    config_data::Config,
    error::{ConfigError, Result},
};

#[derive(Template)]
#[template(path = "config_row.html")]
struct Row {
    panel: String,
    key: String,
    /// All values associated with the key
    vals: Vec<String>,
}

pub async fn row(
    Path((panel, mac)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    _row(panel, mac, state).await
}

pub async fn _row(panel: String, mac: String, mut state: AppState) -> Result<impl IntoResponse> {
    match panel.as_str() {
        "trolley" => {
            state.trolleys.get_latest_data(state.pool.clone()).await;
            match state.trolleys.data.get(&mac) {
                Some((device_name, trolley)) => Ok(Row {
                    panel,
                    key: mac,
                    vals: vec![device_name.to_string(), trolley.to_string()],
                }
                .into_response()),
                None => Err(ConfigError::InvalidKey(mac).into()),
            }
        }
        "ap" => match state.ap.data.get(&mac) {
            Some(ap) => Ok(Row {
                panel,
                key: mac,
                vals: vec![ap.to_string()],
            }
            .into_response()),
            None => Err(ConfigError::InvalidKey(mac).into()),
        },

        _ => Err(ConfigError::InvalidPanel.into()),
    }
}
