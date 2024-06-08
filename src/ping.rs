use std::time::Duration;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{app::AppState, service::MacAddr};

pub async fn ping(
    State(state): State<AppState>,
    Path(mac): Path<String>,
) -> Result<impl IntoResponse> {
    let result = state.services.try_get(&MacAddr(mac));

    // TODO make this a better timeout using interval and tokio::select!
    for _ in 0..15 {
        if result.is_present() {
            let service = result.unwrap();
            let addr = format!("{}/{}", service.addr, service.port);

            let mut stream = TcpStream::connect(addr).await?;
            let mut bytes = Vec::new();

            stream.read_to_end(&mut bytes).await?;

            let loc = String::from_utf8(bytes)?;

            return Ok((StatusCode::FOUND, loc));
        } else if result.is_absent() {
            return Ok((StatusCode::NOT_FOUND, String::from("failed to ping laptop")));
        } else {
            std::thread::sleep(Duration::from_millis(95));
        }
    }

    tracing::error!("failed to obtain lock on services map after 1.5s");
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("failed to ping laptops"),
    ))
}
