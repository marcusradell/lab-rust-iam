use axum::extract::State;
use axum::response::{Html, IntoResponse};
use tower_cookies::Cookies;

use super::router::AppState;

pub async fn handler(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let access_token = cookies
        .get("client_access_token")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or("".to_string());

    if access_token.is_empty() {
        let authorize_url = format!("{}/authorization/authorize", state.api_base_url);
        Html(format!(
            r#"
        <H1>Sign In</H1>
        <a href="{}">Sign in with Rådell</a>
        "#,
            authorize_url
        ))
    } else {
        let sign_out_url = format!("{}/authorization/sign_out", state.api_base_url);
        Html(format!(
            r#"
                <H1>Welcome</H1>
                <p>You are signed in!</p>

                <form action="/client/log_out" method="post">
                    <button type="submit">Log out from client</button>
                </form>

                <p>
                    Go to <a href="{}">Authorization server sign out</a> page.
                </p>
            "#,
            sign_out_url
        ))
    }
}
