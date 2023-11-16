use crate::common::resp::RespVO;
use axum::{ response::IntoResponse, Json};
pub async fn health_check() -> impl IntoResponse {
    let t = "OK".to_string();
    Json(RespVO::<String>::ok(t))
}
