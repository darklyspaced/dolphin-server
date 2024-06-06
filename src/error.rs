use anyhow::Error;
use argon2::password_hash;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

/// Error type used by server
#[derive(Debug)]
pub struct DolphinError(Error);
pub type Result<T> = std::result::Result<T, DolphinError>;

#[derive(Error, Debug)]
/// Error type used to denote all authentication related errors
pub enum AuthError {
    #[error("username {0} was not found")]
    UsernameNotFound(String),
    #[error("password for user {0} was incorrect")]
    PasswordIncorrect(String),
}

#[derive(Error, Debug)]
/// Error type used to denote all authentication related errors
pub enum LocationError {
    #[error("malformed body: {0}")]
    MalformedBody(String),
}

/// Attempts to downcast the error into a type, and acts upon that type. Else, chalks it up to an
/// internal server error.
impl IntoResponse for DolphinError {
    fn into_response(self) -> axum::response::Response {
        match self.0.downcast_ref() {
            Some(err) => match err {
                AuthError::UsernameNotFound(_) | AuthError::PasswordIncorrect(_) => {
                    (StatusCode::FORBIDDEN, "username/password was incorrect").into_response()
                }
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("something catasrophically failed: {}", self.0),
            )
                .into_response(),
        }
    }
}

/// Automatically converts everything that implements `Into<anyhow::Error>` (basically every error
/// type) into Dolphin error to facilitate use of `?`
impl<E> From<E> for DolphinError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
