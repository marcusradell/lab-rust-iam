use argon2::password_hash::SaltString;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::rand_core::OsRng,
};
use axum::{Form, extract::State, response::Redirect};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
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

    let user_id: Uuid = if !user_exists {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
            .to_string();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(&body.email)
        .bind(&password_hash)
        .bind(created_at)
        .execute(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        id
    } else {
        let stored_password_hash: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE email = $1")
                .bind(&body.email)
                .fetch_one(&db)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let parsed_hash = PasswordHash::new(&stored_password_hash)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let argon2 = Argon2::default();
        argon2
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

        let user_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
            .bind(&body.email)
            .fetch_one(&db)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        user_id
    };

    let mut code_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut code_bytes);
    let code = URL_SAFE_NO_PAD.encode(code_bytes);

    let created_at = chrono::Utc::now();
    let expires_at = created_at + chrono::Duration::minutes(10);

    sqlx::query(
        "INSERT INTO authorization_codes (code, user_id, created_at, expires_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&code)
    .bind(user_id)
    .bind(created_at)
    .bind(expires_at)
    .execute(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!(
        "http://localhost:3000/client/authorization_callback?code={}",
        code
    )))
}
