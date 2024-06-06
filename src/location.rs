use axum::response::IntoResponse;
use tracing::debug;

use crate::error::{LocationError, Result};

pub async fn location(body: String) -> Result<impl IntoResponse> {
    let mut lines = body.lines();

    match (lines.next(), lines.next()) {
        (Some(mac), Some(bssid)) => {
            debug!("recieved that {} is connected to {}", mac, bssid);
            Ok(())
        }
        (_, _) => Err(LocationError::MalformedBody(body).into()),
    }
}
