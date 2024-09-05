use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{app::AppState, error::Result, landing::TableRow, service::MacAddr};

#[debug_handler]
pub async fn ping(
    State(state): State<AppState>,
    Path(mac): Path<String>,
) -> Result<impl IntoResponse> {
    let mac_addr = MacAddr(mac.clone());
    let result = state.services.get(mac_addr).await?;
    let (device_name, trolley) = match state.trolleys.data.get(&mac) {
        Some((device_name, trolley)) => (device_name.clone(), trolley.clone()),
        None => (String::new(), String::new()),
    };

    match result {
        Some(service) => {
            let Ok(loc) = service.try_get_loc().await else {
                return Ok(TableRow::error("failed to ping laptop"));
            };

            Ok(TableRow::new(mac, loc.0, device_name, trolley))
        }
        None => Ok(TableRow::error(dbg!(
            "laptop doesn't advertise service anymore"
        ))),
    }
}
