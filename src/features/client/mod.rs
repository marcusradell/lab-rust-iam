use axum::{Router, response::IntoResponse, routing::get};

async fn authorization_callback() -> impl IntoResponse {
    "Welcome!"
}

pub fn router() -> Router {
    Router::new().route("/authorization_callback", get(authorization_callback))
}
