use axum::{extract::State, http::StatusCode, response::Redirect};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use tower_cookies::Cookies;
use uuid::Uuid;

use super::router::AppState;

pub async fn handler(
    cookies: Cookies,
    State(state): State<AppState>,
) -> Result<Redirect, StatusCode> {
    let session_id_cookie: String = cookies
        .get("authorization_session_id")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());

    if session_id_cookie.is_empty() {
        return Ok(Redirect::to("/authorization/sign_in"));
    }

    let user_id =
        Uuid::try_parse(&session_id_cookie).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("user_id: {user_id}");

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
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!(
        "{}/client/authorization_callback?code={}",
        state.api_base_url, code
    )))
}
