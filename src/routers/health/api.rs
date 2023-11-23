use axum::{
    routing::get,
    Router,
};


use super::handler::health_check;


//注册
pub fn health() -> Router {
    //构建注册路由
    Router::new().route("/", get(health_check))
}