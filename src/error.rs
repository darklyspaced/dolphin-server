use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug)]
pub enum DolphinError {
    Auth(AuthError),
    Location(LocationError),
    Generic(anyhow::Error),
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
/// Error type used to denote all location related errors
pub enum LocationError {
    #[error("malformed body: {0}")]
    MalformedBody(String),
}

/// Attempts to downcast the error into a type, and acts upon that type. Else, chalks it up to an
/// internal server error.
impl IntoResponse for DolphinError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        // don't care about any fields regardless of if more are added so `..` is used
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
            },
            Self::Generic(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("something catastrophically failed: {}", self),
            )
                .into_response(),
        }
    }
}

/// Enables all errors to be tries using `?` in the general case
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
