use axum::{
    routing::get,
    Router,
};

use crate::routers::init::handle_router;

use super::handler::health_check;


//注册
pub fn health() -> Router {
    //构建注册路由
    handle_router("/", get(health_check))
}