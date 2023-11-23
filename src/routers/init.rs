use axum::Router;
use axum::routing::MethodRouter;

use mll_axum_utils::middleware::interceptor;
use mll_axum_utils::middleware::jwt::JwtAuth;
use mll_axum_utils::middleware::logger::Logger;

use auth::model;

use super::{auth, health};
use super::vector;
use super::oss;
use super::llm;





//构建路由公共方法
pub fn handle_router(path: String, method_router: MethodRouter) -> Router {
    Router::new().route(path.as_str(), method_router)
}

//api
pub fn routers() -> Router {
    auth_init_router().merge(init_router())
}


//需要权限认证的路由
fn auth_init_router() -> Router {
    let app = Router::new()
        .merge(auth::auth_router()) // 认证模块
        .merge(vector::vector_router()) // 向量模块
        .merge(llm::llm_router()) //AI LLM
        .layer(JwtAuth::<model::Claims>::new(vec!["/login"]))
        // 拦截器拦截黑名单 ip 访问
        .layer(interceptor::blacklist_vec(vec!["127.0.0.1"]))
        // 访问日志记录
        .layer(Logger::default())
        ;
    return app;
}

//不需要权限认证的路由
fn init_router() -> Router {
    let app = Router::new()
        .merge(health::health()) //健康检查
        .merge(oss::oss_router()) // 文件操作
        ;
    return app;
}
