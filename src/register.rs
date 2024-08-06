use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{app::AppState, error::Result};

#[debug_handler]
pub async fn register(
    Path(mac): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();

    sqlx::query!(
        "
INSERT IGNORE INTO laptops (mac)
VALUES (?);
        ",
        mac
    )
    .execute(&pool)
    .await?;

    Ok(())
}
