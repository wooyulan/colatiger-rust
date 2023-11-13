use axum::Json;
use serde::Deserialize;
use axum::response::IntoResponse;

use crate::Response;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

//登录
pub async fn login(Json(req): Json<LoginRequest>) -> impl IntoResponse {
   let str =  Some(format!(
        "Created username: {}, password: {}",
        req.username, req.password
    ));

    Json(Response::ok(str))
}
