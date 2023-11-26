use axum::Router;
use super::{auth, health};
use super::vector;
use super::oss;
use super::llm;

use tower_http::cors::CorsLayer;
//api
pub fn routers() -> Router {

    let cors_layer = CorsLayer::permissive();

    Router::new()
        .nest("/api/v1",auth::auth_router())
        .nest("/api/v1",health::health())
        .nest("/api/v1",vector::vector_router())
        .nest("/api/v1",oss::oss_router())
        .nest("/api/v1",llm::llm_router())
        .layer(cors_layer)

}