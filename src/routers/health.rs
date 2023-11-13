use axum::response::IntoResponse;
use axum::Json;

use crate::Response;

// 健康检查
pub async fn health() -> impl IntoResponse {
    Json(Response::ok(Some("I am ok".to_string())))
}
