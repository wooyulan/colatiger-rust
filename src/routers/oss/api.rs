use axum::Router;
use axum::routing::{post,get};
use axum::extract::DefaultBodyLimit;

use crate::routers::oss::handler::{file_query, file_upload, page_list};



pub fn oss_router() -> Router {
    Router::new()
        .route("/upload", post(file_upload).layer(DefaultBodyLimit::max(1024*1024*30)))
        .route("/query", post(file_query))
        .route("/items",get(page_list))
}
