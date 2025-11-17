use axum::response::{Html, IntoResponse};

pub async fn handler() -> impl IntoResponse {
    Html(
        r#"
        <H1>Sign In</H1>
        <form action="/authorization/sign_in" method="post">
            <div>
                <label for="email">Email</label>
                <input type="email" id="email" name="email" required>
            </div>
        
            <div>
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required>
            </div>
        
            <button type="submit">Sign In</button>
        </form>
        "#,
    )
}
