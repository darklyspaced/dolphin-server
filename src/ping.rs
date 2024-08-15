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
    let mac = MacAddr(mac);
    let result = state.services.get(mac).await?;

    match result {
        Some(service) => {
            let Ok(loc) = service.try_get_loc().await else {
                return Ok(TableRow::error(dbg!("failed to ping laptop")));
            };

            Ok(TableRow::new(String::new(), loc.0))
        }
        None => Ok(TableRow::error(dbg!(
            "laptop doesn't advertise service anymore"
        ))),
    }
}
