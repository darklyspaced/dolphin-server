use askama_axum::{IntoResponse, Template};
use axum::extract::State;

use crate::app::AppState;
use crate::config_data::Config;
use crate::error::Result;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingPage {
    data: Vec<TableRow>,
}

#[derive(Default, Template)]
#[template(path = "row.html")]
/// All the data required for one row of the dashboard
pub struct TableRow {
    pub trolley: String,
    pub device_name: String,
    pub mac: String,
    pub bssid: String,
    pub location: String,
}

impl TableRow {
    pub fn new(mac: String, bssid: String, device_name: String, trolley: String) -> Self {
        Self {
            mac,
            bssid,
            device_name,
            trolley,
            ..Default::default()
        }
    }

    pub fn error(error: &str) -> Self {
        Self {
            location: String::from(error),
            ..Default::default()
        }
    }
}

#[axum::debug_handler]
pub async fn landing(State(mut state): State<AppState>) -> Result<impl IntoResponse> {
    // TODO this redirect doesn't work for some reason
    // Redirect::permanent("/login");
    let locations = state.locations.clone();
    let guard = locations.0.lock().await;

    let mut data = Vec::new();

    state.trolleys.get_latest_data(state.pool.clone()).await;

    for loc in guard.locations.iter() {
        let (device_name, trolley) = match state.trolleys.data.get(&loc.0 .0) {
            Some((device_name, trolley)) => (device_name.clone(), trolley.clone()),
            None => (String::new(), String::new()),
        };
        match loc.1 {
            Some(x) => data.push(TableRow::new(
                loc.0 .0.clone(),
                x.0.clone(),
                device_name,
                trolley,
            )),
            None => data.push(TableRow::new(
                loc.0 .0.clone(),
                String::from(""),
                device_name,
                trolley,
            )),
        }
    }

    Ok(LandingPage { data })
}
