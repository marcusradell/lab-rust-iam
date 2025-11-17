use axum::http::StatusCode;
use axum::response::Html;

pub async fn handler(cookies: tower_cookies::Cookies) -> Result<Html<&'static str>, StatusCode> {
    let session_id = cookies
        .get("authorization_session_id")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());

    if session_id.is_empty() {
        return Ok(Html(
            r#"
    <H1>Sign Out</H1>
    <p>You are already signed out.</p>
        "#,
        ));
    }

    Ok(Html(
        r#"
            <H1>Sign Out</H1>
            <form action="/authorization/sign_out" method="POST">
                <button type="submit">Sign out from authorization server</button>
            </form>
        "#,
    ))
}
