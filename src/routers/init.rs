use axum::Router;
use axum::routing::MethodRouter;

use super::health;
use super::vector;

//构建路由公共方法
pub fn handle_router(path: String, method_router: MethodRouter) -> Router {
   // let path = "/api/v1".to_owned() + path.to_string();
    Router::new().route(path.as_str(), method_router)
}

//api
pub fn routers() -> Router {
    auth_init_router().merge(init_router())
}


//需要权限认证的路由
fn auth_init_router() -> Router {
    let app = Router::new()
        .merge(vector::vector_router()) // 向量模块
        ;
    return app;
}

//不需要权限认证的路由
fn init_router() -> Router {
    let app = Router::new()
        .merge(health::health()) //健康检查
        ;
    return app;
}
