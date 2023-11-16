// pub mod auth;
// pub mod milvus;
//
// mod health;
//
// mod init;
//
// use crate::core::AppState;
// use axum::{routing::get, routing::post, Extension, Router};
// use std::sync::Arc;
//
// // 加载路由
// pub fn load_router(app_state: AppState) -> Router {
//     // 健康检查
//     // let health_router = Router::new().route("/", get(health::health));
//
//     // 登录注册
//     let auth_router = Router::new().route("/auth", post(auth::login));
//
//     // 业务路由
//     let bu_router = Router::new()
//         // milvus
//         .route("/search-pic", get(milvus::search_pic))
//         .route("/embed/pic", post(milvus::insert_pic_vector))
//         ;
//
//
//     // 加入到全局路由
//     Router::new()
//         // .nest("/", health_router)
//         .nest("/open/", auth_router)
//         .nest("/api/v1/", bu_router)
//         .layer(Extension(Arc::new(app_state)))
// }


pub mod init;
mod health;
mod vector;

