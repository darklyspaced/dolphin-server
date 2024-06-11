use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokio::{io::AsyncReadExt, net::TcpStream};

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
    let mut result = services.get(mac).await?;
    match result {
        Some(service) => {
            let addr = format!("{}/{}", service.addr, service.port);
            let mut stream = TcpStream::connect(addr).await?;
            let mut bytes = Vec::new();

            stream.read_to_end(&mut bytes).await?;

            let bssid = String::from_utf8(bytes)?;

            return Ok((StatusCode::FOUND, bssid));
        }
        None => Ok((StatusCode::NOT_FOUND, String::from("failed to ping laptop"))),
    }
}
