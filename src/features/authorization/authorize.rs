use axum::{Form, response::Redirect};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthorizeFormData {
    email: String,
    password: String,
}

pub async fn handler(
    Form(body): Form<AuthorizeFormData>,
) -> Result<Redirect, axum::http::StatusCode> {
    let expected_email = std::env::var("SIGN_IN_EMAIL").unwrap();
    let expected_password = std::env::var("SIGN_IN_PASSWORD").unwrap();

    if body.email != expected_email || body.password != expected_password {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    Ok(Redirect::to(
        "http://localhost:3000/client/authorization_callback?code=123",
    ))
}
