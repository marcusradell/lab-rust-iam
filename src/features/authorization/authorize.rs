use axum::{Form, extract::State, response::Redirect};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthorizeFormData {
    email: String,
    password: String,
}

pub async fn handler(
    State(db): State<PgPool>,
    Form(body): Form<AuthorizeFormData>,
) -> Result<Redirect, axum::http::StatusCode> {
    let user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&body.email)
            .fetch_one(&db)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if !user_exists {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(&body.email)
        .bind(&body.password)
        .bind(created_at)
        .execute(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        let stored_password: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE email = $1")
                .bind(&body.email)
                .fetch_one(&db)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        if body.password != stored_password {
            return Err(axum::http::StatusCode::UNAUTHORIZED);
        }
    }

    Ok(Redirect::to(
        "http://localhost:3000/client/authorization_callback?code=123",
    ))
}
