mod authorize;
mod sign_in;

use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};

async fn token() -> impl IntoResponse {
    "[[TODO: token string]]"
}

pub fn router() -> Router {
    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/authorize", post(authorize::authorize))
        .route("/token", post(token))
}
