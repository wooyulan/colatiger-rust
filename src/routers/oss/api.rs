use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;

use crate::routers::init::handle_router;
use crate::routers::oss::handler::{file_query, file_upload};


pub const VERSION: &str =  "/api/v1";


// 文件上传
fn upload() -> Router {
    handle_router(VERSION.to_string()+"/upload", post(file_upload)).layer(DefaultBodyLimit::max(1024*1024*30))
}

fn find_obj()-> Router {
    handle_router(VERSION.to_string()+"/query", post(file_query))
}


pub fn oss_router() -> Router {
    Router::new()
        .merge(upload())
        .merge(find_obj())
}
