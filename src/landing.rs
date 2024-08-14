use askama_axum::{IntoResponse, Template};
use axum::extract::State;

use crate::app::AppState;
use crate::error::Result;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingPage {
    data: Vec<TableData>,
}

#[derive(Default)]
struct TableData {
    pub trolley: char,
    pub device_name: String,
    pub mac: String,
    pub bssid: String,
    pub location: String,
}

impl TableData {
    pub fn new(mac: String, bssid: String) -> Self {
        Self {
            mac,
            bssid,
            ..Default::default()
        }
    }
}

#[axum::debug_handler]
pub async fn landing(State(state): State<AppState>) -> Result<impl IntoResponse> {
    // TODO this redirect doesn't work for some reason
    // Redirect::permanent("/login");
    let locations = state.locations.clone();
    let guard = locations.0.lock().await;

    let mut data = Vec::new();

    for loc in guard.locations.iter() {
        match loc.1 {
            Some(x) => data.push(TableData::new(loc.0 .0.clone(), x.0.clone())),
            None => data.push(TableData::new(loc.0 .0.clone(), String::from(""))),
        }
    }

    Ok(LandingPage { data })
}
