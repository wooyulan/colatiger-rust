pub mod auth;
pub mod health;
pub mod milvus;

use axum::{routing::get, routing::post, Router, Extension};

use crate::core::AppState;

// 加载路由
pub fn load_router(app_state: AppState) -> Router {
    // 健康检查
    let health_router = Router::new().route("/", get(health::health));

    // 登录注册
    let auth_router = Router::new().route("/auth", post(auth::login));

    // 业务路由
    let bu_router = Router::new()
        // milvus
        .route("/has", get(milvus::test_milvus));

    // 加入到全局路由
    Router::new()
        .nest("/", health_router)
        .nest("/open/", auth_router)
        .nest("/api/v1/", bu_router)
        .layer(Extension(app_state))
}
