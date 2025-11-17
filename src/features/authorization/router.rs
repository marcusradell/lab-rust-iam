use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .route("/sign_in", get(super::sign_in::handler))
        .route("/sign_in", post(super::authorize::handler))
        .route("/authorize", post(super::authorize::handler))
        .route("/token", post(super::token::handler))
        .with_state(db)
}
