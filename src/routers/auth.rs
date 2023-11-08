use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

//登录
pub async fn login(Json(req): Json<LoginRequest>) -> String {
    format!(
        "Created username: {}, password: {}",
        req.username, req.password
    )
}
