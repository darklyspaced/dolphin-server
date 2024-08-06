use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::AppState,
    error::Result,
    service::{MacAddr, Services},
};

pub async fn ping(
    State(state): State<AppState>,
    Path(mac): Path<String>,
) -> Result<impl IntoResponse> {
    _ping(MacAddr(mac), state.services).await
}

async fn _ping(mac: MacAddr, services: Services) -> Result<impl IntoResponse> {
    let result = services.get(mac).await?;
    match result {
        Some(service) => {
            let Ok(loc) = service.try_get_loc().await else {
                return Ok((StatusCode::NOT_FOUND, String::from("failed to ping laptop")));
            };

            Ok((StatusCode::FOUND, loc))
        }
        None => Ok((StatusCode::NOT_FOUND, String::from("failed to ping laptop"))),
    }
}
