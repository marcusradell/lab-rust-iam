#[cfg(test)]
mod router_test;

use axum::{Router, http::StatusCode, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(status))
}

async fn status() -> StatusCode {
    StatusCode::OK
}
