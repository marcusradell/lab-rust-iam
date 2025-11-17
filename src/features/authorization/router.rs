use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub api_base_url: String,
}

pub fn router(db: PgPool, api_base_url: String) -> Router {
    let state = AppState { db, api_base_url };

    Router::new()
        .route("/authorize", get(super::authorize::handler))
        .route("/sign_in", get(super::sign_in_page::handler))
        .route("/sign_in", post(super::sign_in::handler))
        .route("/sign_out", get(super::sign_out_page::handler))
        .route("/sign_out", post(super::sign_out::handler))
        .route("/token", post(super::token::handler))
        .with_state(state)
        .layer(CookieManagerLayer::new())
}
