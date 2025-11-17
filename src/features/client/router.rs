use axum::{
    Router,
    routing::{get, post},
};
use tower_cookies::CookieManagerLayer;

#[derive(Clone)]
pub struct AppState {
    pub api_base_url: String,
}

pub fn router(api_base_url: String) -> Router {
    let state = AppState { api_base_url };

    Router::new()
        .route(
            "/authorization_callback",
            get(super::authorization_callback::handler),
        )
        .route("/", get(super::landing_page::handler))
        .route("/log_out", post(super::log_out::handler))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
