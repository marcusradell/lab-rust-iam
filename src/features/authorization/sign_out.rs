use axum::{http::StatusCode, response::Redirect};
use tower_cookies::{Cookie, Cookies};

pub async fn handler(cookies: Cookies) -> Result<Redirect, StatusCode> {
    let mut cookie = Cookie::from("authorization_session_id");
    cookie.set_path("/authorization");
    cookies.remove(cookie);

    Ok(Redirect::to("/authorization/sign_out"))
}
