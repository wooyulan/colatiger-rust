pub mod auth;
pub mod health;

use axum::{Router,routing::get,routing::post};


// 加载路由
pub fn load_router() -> Router {
    // 健康检查
    let health_router = Router::new()
    .route("/", get(health::health));

    // 登录注册
    let auth_router = Router::new().route("/auth",post(auth::login));

    // 加入到全局路由
    Router::new()
    .nest("/", health_router)
    .nest("/api/v1/", auth_router)
}


