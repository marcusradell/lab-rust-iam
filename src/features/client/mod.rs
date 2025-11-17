use axum::response::{Html, IntoResponse, Redirect};
use tower_cookies::{Cookie, Cookies};
mod authorization_callback;
mod router;
pub use router::router;

async fn landing_page(cookies: Cookies) -> impl IntoResponse {
    let access_token = cookies
        .get("client_access_token")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or("".to_string());

    if access_token.is_empty() {
        Html(
            r#"
        <H1>Sign In</H1>
        <a href="http://localhost:3000/authorization/authorize">Sign in with Rådell</a>
        "#,
        )
    } else {
        Html(
            r#"
                <H1>Welcome</H1>
                <p>You are signed in!</p>

                <form action="/client/log_out" method="post">
                    <button type="submit">Log out from client</button>
                </form>

                <p>
                    Go to <a href="http://localhost:3000/authorization/sign_out">Authorization server sign out</a> page.
                </p>
            "#,
        )
    }
}

async fn log_out(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::from("client_access_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    let mut cookie = Cookie::from("client_refresh_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    Redirect::to("/client")
}
