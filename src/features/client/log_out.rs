use axum::response::{IntoResponse, Redirect};
use tower_cookies::{Cookie, Cookies};

pub async fn handler(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::from("client_access_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    let mut cookie = Cookie::from("client_refresh_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    Redirect::to("/client")
}
