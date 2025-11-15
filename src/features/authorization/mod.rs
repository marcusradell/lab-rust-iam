use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};

async fn authorize() -> impl IntoResponse {
    Redirect::to("/client/authorization_callback?code=123")
}

async fn token() -> impl IntoResponse {
    "[[TODO: token string]]"
}

pub fn router() -> Router {
    Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
}
