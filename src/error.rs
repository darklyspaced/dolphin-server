use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse};

/// Error type used by server
#[derive(Debug)]
pub struct DolphinError(Error);
pub type Result<T> = std::result::Result<T, DolphinError>;

impl IntoResponse for DolphinError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("something catasrophically failed: {}", self.0),
        )
            .into_response()
    }
}

/// Automatically converts everything that implements Into<anyhow::Error> (basically every error
/// type) into Dolphin error to facilitate use of `?`
impl<E> From<E> for DolphinError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
