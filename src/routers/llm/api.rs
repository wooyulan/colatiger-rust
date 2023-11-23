use std::string::ToString;
use axum::Router;
use axum::routing::{get, post};
use crate::routers::init::handle_router;
use crate::routers::llm::handler::chat_stream;


pub const VERSION: &str =  "/api/v1";

// SSE LLM
fn llm_sse() -> Router {
    //构建注册路由
    handle_router(VERSION.to_string()+"/llm/stream", get(chat_stream))
}

// 向量路由
pub fn llm_router() -> Router {
    Router::new()
        .merge(llm_sse())
}
