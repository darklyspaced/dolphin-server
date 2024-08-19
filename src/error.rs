use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::fmt::Display;
use thiserror::Error;
use tracing::{error, warn};

use crate::service::MacAddr;

pub type Result<T> = std::result::Result<T, DolphinError>;

macro_rules! define {
    ([$($error_ty:ident),+]) => {
        /// Enables all errors to be tried using `?`
        impl<E> From<E> for DolphinError
        where
            E: Into<anyhow::Error>,
        {
            // allow generic errors to be converted automatically
            default fn from(value: E) -> Self {
                Self::Generic(value.into())
            }
        }

        /// Error type that underpins entire application.
        #[derive(Debug)]
        pub enum DolphinError {
            $($error_ty($error_ty),)+
            Generic(anyhow::Error),
        }

        impl Display for DolphinError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$error_ty(e) => write!(f, "{}", e),)+
                    Self::Generic(e) => write!(f, "{}", e),
                }
            }
        }
    };

    (
        $( [$($error_ty:ident),*] )?

        pub enum $name:ident {
        $(
            #[$err_msg:meta]
            $variant:ident $(
                ( $($ty:ty),+ $(,)? ), // handle enum anon arguments + optional trailing comma
                // gonna pretend named fields don't exist
            )? $(,)?
        )+
        } $($tail:tt)*
    ) =>
    {
        #[derive(Debug, Error)]
        pub enum $name {
        $(
            #[$err_msg]
            $variant $(
                ( $($ty,)+ ) // need brackets for sanitation
            )? ,
        )+
        }

        impl From<$name> for DolphinError {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }

        define!{
            $([$($error_ty,)* $name])?

            $($tail)*
        }
    };
}

define! {
    []

    pub enum AuthError {
        #[error("username {0} was not found")]
        UsernameNotFound(String),
        #[error("password for user {0} was incorrect")]
        PasswordIncorrect(String),
    }

    pub enum LocationError {
        #[error("malformed body: {0}")]
        MalformedBody(String),
        #[error(
            "failed to obtain lock on services after 100ms where requesting: {}",
            0.0
        )]
        LockFailed(MacAddr),
    }

    pub enum ConfigError {
        #[error("there is no config for this path")]
        InvalidPanel,
    }
}

/// Attempts to downcast the error into a type and then acts upon that type. If it's a general
/// error then we chalk it up to a server error; the client doesn't need to know the details.
impl IntoResponse for DolphinError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::AuthError(err) => match err {
                AuthError::UsernameNotFound(..) | AuthError::PasswordIncorrect(..) => {
                    warn!("{}", err);
                    (StatusCode::FORBIDDEN, "username/password was incorrect").into_response()
                }
            },
            Self::LocationError(err) => match err {
                LocationError::MalformedBody(..) => {
                    error!("{}", err);
                    (
                        StatusCode::BAD_REQUEST,
                        "recieved malformed location from client",
                    )
                        .into_response()
                }
                LocationError::LockFailed(ref addr) => {
                    error!("{}", &err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        String::from("failed to ping ") + &addr.0,
                    )
                        .into_response()
                }
            },
            Self::ConfigError(err) => match err {
                ConfigError::InvalidPanel => {
                    error!("{}", err);
                    (StatusCode::NOT_FOUND, "invalid config path, try again").into_response()
                }
            },
            Self::Generic(err) => {
                error!("UNEXPECTED: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "something catastrophically failed",
                )
                    .into_response()
            }
        }
    }
}
