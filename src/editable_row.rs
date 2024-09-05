use askama_axum::{IntoResponse, Template};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{app::AppState, config_data::Config, error::Result};

#[derive(Template)]
#[template(path = "editable_row.html")]
struct EditableRow {
    panel: String,
    /// Key that is ID for the row
    key: String,
    /// Row of values associated with key and input names
    row: Vec<(String, String)>,
}

pub async fn editable_row(
    State(mut state): State<AppState>,
    Path((panel, mac)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match panel.as_str() {
        "trolley" => {
            state.trolleys.get_latest_data(state.pool.clone()).await;
            match state.trolleys.data.get(&mac) {
                Some((device_name, trolley)) => Ok(EditableRow {
                    panel,
                    key: mac,
                    row: vec![
                        (device_name.clone(), String::from("device_name")),
                        (trolley.clone(), String::from("trolley")),
                    ],
                }
                .into_response()),
                None => {
                    unreachable!("csv file was uploaded yet row doesn't exist");
                }
            }
        }
        "ap" => todo!(),
        _ => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}
