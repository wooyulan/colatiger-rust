use axum::response::IntoResponse;
use axum::Json;

use crate::Result;

pub async fn health() -> impl IntoResponse {
    let data = Some("I am ok".to_string());

    let response = Result::ok(data);

    Json(response)
}
