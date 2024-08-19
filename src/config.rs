use crate::{
    app::AppState,
    config_data::Config,
    error::{ConfigError, Result},
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Html,
};

#[derive(Template)]
#[template(path = "config.html", print = "code")]
struct TrolleysPage {
    data: Vec<[String; 3]>,
}

#[derive(Template)]
#[template(path = "config.html", print = "code")]
struct ApPage {
    data: Vec<[String; 2]>,
}

/// Renders the config based on what config page is being looked at
///
/// Note: `Html()` needs to be explicity returned since ConfigPage is generic over Config and
/// different instances of Config are considered different types :(
pub async fn config(
    Path(panel): Path<String>,
    State(mut state): State<AppState>,
) -> Result<impl IntoResponse> {
    match panel.as_str() {
        "ap" => {
            state.ap.get_latest_data(state.pool.clone()).await;

            Ok(Html(
                ApPage {
                    data: state.ap.into_iter().collect::<Vec<_>>(),
                }
                .render()
                .unwrap(),
            ))
        }
        "trolley" => {
            state.trolleys.get_latest_data(state.pool.clone()).await;

            Ok(Html(
                TrolleysPage {
                    data: state.trolleys.into_iter().collect::<Vec<_>>(),
                }
                .render()
                .unwrap(),
            ))
        }
        _ => Err(ConfigError::InvalidPanel.into()),
    }
}
