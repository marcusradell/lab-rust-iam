use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};

async fn authorize() -> impl IntoResponse {
    Redirect::to("/client/authorization_callback?code=123")
}

pub fn router() -> Router {
    Router::new().route("/", get(authorize))
}
