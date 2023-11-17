use axum::Json;
use axum::response::IntoResponse;
use crate::common::resp::RespVO;

// 注册
pub async fn register() -> impl IntoResponse{
    Json(RespVO::<String>::ok_empty())
}