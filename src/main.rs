mod features;

use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};
use sqlx::{migrate, postgres::PgPoolOptions};

async fn root_route() -> impl IntoResponse {
    Redirect::to("/client")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url =
        std::env::var("DATABASE_URL").map_err(|e| format!("DATABASE_URL must be set: {}", e))?;

    let db = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    migrate!("./migrations")
        .run(&db)
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    let app = Router::new()
        .nest("/status", features::status::router())
        .nest(
            "/authorization",
            features::authorization::router(db.clone()),
        )
        .nest("/client", features::client::router())
        .merge(Router::new().route("/", get(root_route)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| format!("Failed to bind to address: {}", e))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {}", e))?;

    Ok(())
}
