use axum::{http::HeaderMap, response::Redirect};

pub async fn landing(_headers: HeaderMap) -> Redirect {
    // TODO this redirect doesn't work for some reason
    Redirect::permanent("/login")
}
