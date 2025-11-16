use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Data {
    code: String,
}

#[derive(Serialize)]
pub struct Response {
    access_token: String,
}

pub async fn handler(Json(data): Json<Data>) -> impl IntoResponse {
    Json(Response {
        access_token: format!("TODO: access token based on auth code: {}", data.code),
    })
}
