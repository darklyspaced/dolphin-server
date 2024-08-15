use askama_axum::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    app::AppState,
    error::Result,
    service::{MacAddr, Services},
};

#[derive(Template, Default)]
#[template(path = "new_loc.html")]
struct NewLoc {
    location: String,
    bssid: String,
}

impl NewLoc {
    pub fn new(location: &str, bssid: &str) -> Self {
        Self {
            location: String::from(location),
            bssid: String::from(bssid),
        }
    }

    pub fn error(error: &str) -> Html<String> {
        Html(
            Self {
                location: String::from(error),
                bssid: String::from(error),
            }
            .render()
            .unwrap(),
        )
    }
}

#[debug_handler]
pub async fn ping(
    State(state): State<AppState>,
    Path(mac): Path<String>,
) -> Result<(StatusCode, Html<String>)> {
    let mac = MacAddr(mac);
    let result = state.services.get(mac).await?;

    match result {
        Some(service) => {
            let Ok(loc) = service.try_get_loc().await else {
                return Ok((
                    StatusCode::NOT_FOUND,
                    NewLoc::error(dbg!("failed to ping laptop")),
                ));
            };

            Ok((
                StatusCode::FOUND,
                axum::response::Html(
                    NewLoc {
                        location: String::new(),
                        bssid: loc.0,
                    }
                    .render()
                    .unwrap(),
                ),
            ))
        }
        None => Ok((
            StatusCode::NOT_FOUND,
            NewLoc::error(dbg!("laptop doesn't advertise service anymore")),
        )),
    }
}
