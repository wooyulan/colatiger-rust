use axum::Router;
use axum::routing::get;
use crate::routers::llm::handler::chat_stream;


// 向量路由
pub fn llm_router() -> Router {
    Router::new()
        .route("/llm/stream", get(chat_stream))
}
