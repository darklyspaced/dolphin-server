use askama_axum::{IntoResponse, Template};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{app::AppState, error::Result};

#[derive(Template)]
#[template(path = "editable_row.html")]
struct EditableRow {
    /// Key that is ID for the row
    key: String,
    /// Row of values that should be editable
    row: Vec<String>,
}

pub async fn editable_row(
    State(state): State<AppState>,
    Path((panel, mac)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match panel.as_str() {
        "trolley" => match state.trolleys.data.get(&mac) {
            Some((device_name, trolley)) => Ok(EditableRow {
                key: mac.clone(),
                row: vec![mac, device_name.clone(), trolley.clone()],
            }
            .into_response()),
            None => unreachable!("csv file was uploaded yet row doesn't exist"),
        },
        "ap" => todo!(),
        _ => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}
