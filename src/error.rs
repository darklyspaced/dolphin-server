use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Display;
use thiserror::Error;

use crate::service::MacAddr;

pub type Result<T> = std::result::Result<T, DolphinError>;

#[derive(Debug)]
/// Error type that underpins entire application.
pub enum DolphinError {
    Auth(AuthError),
    Location(LocationError),
    Generic(anyhow::Error),
}

#[derive(Error, Debug)]
/// Error type used to denote all authentication related errors
pub enum AuthError {
    #[error("username {0} was not found")]
    UsernameNotFound(String),
    #[error("password for user {0} was incorrect")]
    PasswordIncorrect(String),
}

#[derive(Error, Debug)]
/// Error type used to denote all location related errors
pub enum LocationError {
    #[error("malformed body: {0}")]
    MalformedBody(String),
    #[error(
        "failed to obtain lock on services after 100ms where requesting: {}",
        0.0
    )]
    LockFailed(MacAddr),
}

/// Attempts to downcast the error into a type and then acts upon that type. If it's a general
/// error then we chalk it up to a server error; the client doesn't need to know the details.
impl IntoResponse for DolphinError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Self::Auth(err) => match err {
                AuthError::UsernameNotFound(..) | AuthError::PasswordIncorrect(..) => {
                    tracing::warn!("{}", err);
                    (StatusCode::FORBIDDEN, "username/password was incorrect").into_response()
                }
            },
            Self::Location(err) => match err {
                LocationError::MalformedBody(..) => {
                    tracing::error!("{}", err);
                    (
                        StatusCode::BAD_REQUEST,
                        "recieved malformed location from client",
                    )
                        .into_response()
                }
                LocationError::LockFailed(ref addr) => {
                    tracing::error!("{}", &err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        String::from("failed to ping ") + &addr.0,
                    )
                        .into_response()
                }
            },
            Self::Generic(err) => {
                tracing::error!("UNEXPECTED: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "something catastrophically failed",
                )
                    .into_response()
            }
        }
    }
}

impl Display for DolphinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth(err) => write!(f, "{}", err),
            Self::Location(err) => write!(f, "{}", err),
            Self::Generic(err) => write!(f, "{}", err),
        }
    }
}

/// Enables all errors to be tried using `?`
impl<E> From<E> for DolphinError
where
    E: Into<anyhow::Error>,
{
    default fn from(value: E) -> Self {
        Self::Generic(value.into())
    }
}

impl From<AuthError> for DolphinError {
    fn from(value: AuthError) -> Self {
        Self::Auth(value)
    }
}

impl From<LocationError> for DolphinError {
    fn from(value: LocationError) -> Self {
        Self::Location(value)
    }
}
