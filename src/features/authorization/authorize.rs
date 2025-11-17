use axum::{http::StatusCode, response::Redirect};

pub async fn handler() -> Result<Redirect, StatusCode> {
    Ok(Redirect::to("/authorization/sign_in"))
}
