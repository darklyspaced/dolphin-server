use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tracing::debug;

use crate::{
    app::AppState,
    error::{LocationError, Result},
};

pub async fn location(State(state): State<AppState>, body: String) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();
    let mut lines = body.lines();

    match (lines.next(), lines.next()) {
        (Some(mac), Some(bssid)) => {
            debug!("recieved that {} is connected to {}", mac, bssid);

            sqlx::query!(
                "
INSERT INTO locations (mac, bssid)
VALUES (?, ?)
ON DUPLICATE KEY UPDATE
bssid = VALUES(bssid);
                ",
                mac,
                bssid
            )
            .execute(&pool)
            .await?;

            Ok(StatusCode::CREATED)
        }
        (_, _) => Err(LocationError::MalformedBody(body).into()),
    }
}
