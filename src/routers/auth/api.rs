use axum::Router;
use axum::routing::get;
use crate::routers::auth::handler::register;
use crate::routers::init::handle_router;


pub const VERSION: &str =  "/api/v1";

// 注册
fn reg() -> Router {
    //构建注册路由
    handle_router(VERSION.to_string()+"/reg", get(register))
}


// 向量路由
pub fn auth_router() -> Router {
    Router::new()
        .merge(reg())
}
