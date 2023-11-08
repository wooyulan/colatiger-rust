use axum::response::IntoResponse;
use axum::Json;

use crate::Result;

// 健康检查
pub async fn health() -> impl IntoResponse {
    Json(Result::ok(Some("I am ok".to_string())))
}
