use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;
use serde::Serialize;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use super::router::AppState;

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    code: String,
}

#[derive(Deserialize)]
struct TokenData {
    access_token: String,
    refresh_token: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Query(query_data): Query<QueryData>,
    cookies: Cookies,
) -> Result<Redirect, axum::http::StatusCode> {
    let client = reqwest::Client::new();

    let token_url = format!("{}/authorization/token", state.api_base_url);

    let response = client
        .post(&token_url)
        .json(&query_data)
        .send()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data: TokenData = response
        .json()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_cookie: Cookie = Cookie::build(("client_access_token", token_data.access_token))
        .path("/client")
        .secure(true)
        .http_only(true)
        .into();

    cookies.add(access_cookie);

    let refresh_cookie: Cookie = Cookie::build(("client_refresh_token", token_data.refresh_token))
        .path("/client")
        .secure(true)
        .http_only(true)
        .into();

    cookies.add(refresh_cookie);

    Ok(Redirect::to("/client"))
}
